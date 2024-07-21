#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"
#include "../utils/vector.h"



Tokens readTokens(const char *filepath) {
    FILE *file = fopen(filepath, "r");
    if (file == NULL) {
        setError(ERROR_FILE_OPEN, strerror(errno));
        return (Tokens){0};
    }

    Tokenizer tokensizer = {0};
    tokensizer.source = file;
    Tokens tokens = {0};
    Token token;
    while (tryReadToken(&tokensizer, &token)) {
        addTokens(&tokens, token);
    }

    deleteTokenizer(&tokensizer);
    fclose(file);
    return tokens;
}

// usize annotationLength(Tokens tokens, usize index) {
//     VECTOR_BOUNDS_CHECK(tokens, index, 1)
   
//     Token startToken = tokens.data[index];
//     if (startToken.type != TOKEN_TYPE_KEYWORD) {
//         return 0;
//     }
//     if (startToken.keyword != KEYWORD_BRACKET_SQUARE_OPEN) {
//         return 0;
//     }
//     for (usize i = index; i < tokens.size; i++){
//         if (tokens.data[i].type != TOKEN_TYPE_KEYWORD) {
//             continue;
//         }
//         if (tokens.data[i].keyword == KEYWORD_BRACKET_SQUARE_CLOSE) {
//             return i - index + 1;
//         }
//     }
//     char msg[70];
//     snprintf(msg, 70, "\"[\"@%d:%ld has no closing bracket.", startToken.position.line, startToken.position.character);
//     setError(ERROR_BRACKET_MISSMATCH, msg);
//     return 1;
// }

// bool __attribute__((const))isAnnotation(Token token) {
//     if (token.type != TOKEN_TYPE_KEYWORD) {
//         return false;
//     }
//     return token.keyword == KEYWORD_BRACKET_SQUARE_OPEN;
// }

//
// Keyword properties with fallthrough, partial switches
//

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wswitch"
#pragma GCC diagnostic ignored "-Wswitch-enum"

bool __attribute__((const))isPrefixToken(Token token) {
    if (token.type != TOKEN_TYPE_KEYWORD) {
        return false;
    }
    switch (token.keyword) {
        case KEYWORD_CONST:
        case KEYWORD_EXPORT:
        case KEYWORD_EXTERN:
        case KEYWORD_INLINE:
        case KEYWORD_LOCAL:
        case KEYWORD_PUBLIC:
        case KEYWORD_SEALED:
        case KEYWORD_STAGED:
        case KEYWORD_STATIC:
        case KEYWORD_UNSAFE:
            return true;  
    } 
    return false;
}

bool __attribute__((const))isTopLevelToken(Token token) {
    if (token.type == TOKEN_TYPE_IDENTIFIER) {
        return true;
    }

    switch (token.keyword) {
        case KEYWORD_ENUM:
        case KEYWORD_FN:
        case KEYWORD_IMPLEMENT:
        case KEYWORD_IMPORT:
        case KEYWORD_INTERFACE:
        case KEYWORD_NAMESPACE:
        case KEYWORD_STRUCT:
        case KEYWORD_TYPE:
        case KEYWORD_UNION:
            return true;
    }
    return false;
}

bool isAccessModifier(Token token) {
    if (token.type != TOKEN_TYPE_KEYWORD) {
        return false;
    }
    switch (token.keyword) {
        case KEYWORD_PUBLIC:
        case KEYWORD_LOCAL:
            return true;
        default:
            return false;
    }
}

CREATE_VECTOR_TYPE(OpenIndices, usize, NULL)

usize closingBracket(Tokens tokens, usize start) {
    OpenIndices openBracketStack = {0};
    for (usize i = start; i < tokens.size; i++) {
        if (tokens.data[i].type != TOKEN_TYPE_KEYWORD) {
            continue;
        }
        Keyword keyword = tokens.data[i].keyword;
        switch (keyword) {
            case KEYWORD_BRACKET_CURLY_OPEN:
            case KEYWORD_BRACKET_DIAMOND_OPEN:
            case KEYWORD_BRACKET_ROUND_OPEN:
            case KEYWORD_BRACKET_SQUARE_OPEN:
                addOpenIndices(&openBracketStack, i);
                continue;
        }
        switch (keyword) {
            case KEYWORD_BRACKET_CURLY_CLOSE:
            case KEYWORD_BRACKET_DIAMOND_CLOSE:
            case KEYWORD_BRACKET_ROUND_CLOSE:
            case KEYWORD_BRACKET_SQUARE_CLOSE:
                if (openBracketStack.size <= 0) {
                    char msg[100];
                    snprintf(msg, 100, "Unmatched bracket @%d:%ld", tokens.data[i].position.line, tokens.data[i].position.character);
                    setError(ERROR_BRACKET_MISSMATCH, msg);
                    goto errorCleanup;
                }
                usize openingIndex = popOpenIndices(&openBracketStack);
                if (keyword - 1 != tokens.data[openingIndex].keyword) {
                    char msg[100];
                    snprintf(msg, 100, "Unmatched bracket @%d:%ld", tokens.data[i].position.line, tokens.data[i].position.character);
                    setError(ERROR_BRACKET_MISSMATCH, msg);
                    goto errorCleanup;
                }
                if (openBracketStack.size == 0) {
                    deleteOpenIndices(&openBracketStack);
                    return i;
                }
        }



    }
    //only reached on unmatched brackets
    usize openIndex = popOpenIndices(&openBracketStack);
    char msg[100];
    snprintf(msg, 100, "Unmatched bracket @%d:%ld", tokens.data[openIndex].position.line, tokens.data[openIndex].position.character);
    setError(ERROR_BRACKET_MISSMATCH, msg);
errorCleanup:
    deleteOpenIndices(&openBracketStack);
    return 0;
    
}

usize parseSecondLevel(Tokens tokens, usize start, AstNode *parent) {
    
}

usize parseNamespace(Tokens tokens, usize index, usize skippedTokens, AstNode *parent) {
    AstNode *node = MAKE_NODE(NamespaceNode);
    NamespaceNode *data = GET_NODE_DATA(NamespaceNode, node);
    data->accessModifier = ACCESS_MODIFIER_DEFAULT_PRIVATE;

    if (tokens.size - index < 3) {
        setError(ERROR_INCOMPLETE_STATEMENT, "Not enough tokens for Keyword.");
        goto errorCleanup;
    }
    if (skippedTokens > 0 && isAccessModifier(tokens.data[index - 1])) {
        switch (tokens.data[index -1].keyword) {
            case KEYWORD_LOCAL:
                data->accessModifier = ACCESS_MODIFIER_LOCAL;
                break;
            case KEYWORD_PUBLIC:
                data->accessModifier = ACCESS_MODIFIER_PUBLIC;
                break;
            default:
                setError(ERROR_UNREACHABLE, "Unreachable");
                goto errorCleanup;
        }
    }
    Token nextToken = tokens.data[index + 1];
    if (nextToken.type != TOKEN_TYPE_IDENTIFIER) {
        char msg[100];
        snprintf(msg, 100, "Expected identier @%d:%ld", nextToken.position.line, nextToken.position.character);
        setError(ERROR_INVALID_TOKEN, msg);
        goto errorCleanup;
    }
    STRCPY(data->name, nextToken.identifier.name);
    Token nextNextToken = tokens.data[index + 2];
    if (nextNextToken.type != TOKEN_TYPE_KEYWORD) {
        char msg[100];
        snprintf(msg, 100, "Expected keyword (';', '{') @%d:%ld", nextToken.position.line, nextToken.position.character);
        setError(ERROR_INVALID_TOKEN, msg);
        goto errorCleanup;
    }
    usize length = 3;
    switch (nextNextToken.keyword) {
        case KEYWORD_BRACKET_CURLY_OPEN:
            length++;
            __attribute__ ((fallthrough));
        case KEYWORD_COLON_SEMI:
            length += parseSecondLevel(tokens, index + 3, node);
            break;
        default:
            char msg[100];
            snprintf(msg, 100, "Expected keyword (';', '{') @%d:%ld", nextToken.position.line, nextToken.position.character);
            setError(ERROR_INVALID_TOKEN, msg);
            goto errorCleanup;
    }
    node->parent = parent;
    addAstNodes(&parent->children, node);
    return length;

errorCleanup:
    deleteNode(node);
    return 0;
}


usize parseTopLevel(Tokens tokens, usize index, usize skippedTokens, AstNode *parent) {
    VECTOR_BOUNDS_CHECK(tokens, index, 0)
    Token topLevelToken = tokens.data[index];
    if (topLevelToken.type != TOKEN_TYPE_KEYWORD) {
        //invalid token type
        return 0;
    }
    switch (topLevelToken.keyword) {
        case KEYWORD_ENUM:

        case KEYWORD_FN:

        case KEYWORD_IMPLEMENT:

        case KEYWORD_IMPORT:

        case KEYWORD_INTERFACE:

        case KEYWORD_NAMESPACE:
            return parseNamespace(tokens, index, skippedTokens, parent);
        case KEYWORD_STRUCT:

        case KEYWORD_TYPE:

        case KEYWORD_UNION:

        default:
            setError(ERROR_UNREACHABLE, "Unreachable");
            return 0;
    }
}

#pragma GCC diagnostic pop

ParseResult parseFile(const char *filepath) {
    AstNode *root = createNode(NODE_TYPE_FILE_ROOT);
    FileRootNode *fileRootData = GET_NODE_DATA(FileRootNode, root);
    STRCPY(fileRootData->path, filepath);
   
    Tokens tokens = readTokens(filepath);
    if (error()) {
        goto errorCleanup;
    }

    usize skippedTokens = 0;
    for (usize i = 0; i < tokens.size; i++) {
        if (isPrefixToken(tokens.data[i])) {
            skippedTokens++;
            continue;
        }

        // if (isAnnotation(tokens.data[i])) {
        //     usize length = annotationLength(tokens, i);
        //     if (error()) {
        //         goto errorCleanup;
        //     }
        //     skippedTokens += length;
        //     i += length;
        // }
        
        if (isTopLevelToken(tokens.data[i])) {
            usize length = parseTopLevel(tokens, i, skippedTokens, root);
            if (error()) {
                goto errorCleanup;
            }
            skippedTokens = 0;
            i += length;
            continue;
        }
        char msg[100];
        switch (tokens.data[i].type) {
            case TOKEN_TYPE_IDENTIFIER:
                snprintf(msg, 100, "Invalid TopLevelToken:%s", tokens.data[i].identifier.name);
                break;
            case TOKEN_TYPE_KEYWORD:
                snprintf(msg, 100, "Invalid TopLevelToken:%d", tokens.data[i].keyword);
                break;
            default:
                snprintf(msg, 100, "Unknown token type");
                break;
        }
        setError(ERROR_INVALID_TOKEN, msg);
        return (ParseResult) {0};
    }
    return (ParseResult) { .tokens = tokens, .root = root};

errorCleanup:
    deleteNode(root);
    deleteTokens(&tokens);
    return (ParseResult) {0};
}


void deleteParserResult(ParseResult result) {
    deleteTokens(&result.tokens);

}
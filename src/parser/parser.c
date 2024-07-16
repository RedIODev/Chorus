#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"
#include "lexer.h"
#include "../utils/vector.h"

void deleteToken(Token token) {
    if (token.type == TOKEN_TYPE_IDENTIFIER) {
        free(token.identifier.name);
    }
}

CREATE_VECTOR_TYPE(Tokens, Token, deleteToken)

Token takeToken(Tokens tokens, usize index) {
    VECTOR_BOUNDS_CHECK(tokens, index, (Token) {0})
    Token result = tokens.data[index];
    tokens.data[index] = (Token) {0};
    return result;
}

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

usize annotationLength(Tokens tokens, usize index) {
    VECTOR_BOUNDS_CHECK(tokens, index, 1)
   
    Token startToken = tokens.data[index];
    if (startToken.type != TOKEN_TYPE_KEYWORD) {
        return 0;
    }
    if (startToken.keyword != KEYWORD_BRACKET_SQUARE_OPEN) {
        return 0;
    }
    for (usize i = index; i < tokens.size; i++){
        if (tokens.data[i].type != TOKEN_TYPE_KEYWORD) {
            continue;
        }
        if (tokens.data[i].keyword == KEYWORD_BRACKET_SQUARE_CLOSE) {
            return i - index + 1;
        }
    }
    char msg[70];
    snprintf(msg, 70, "\"[\"@%d:%ld has no closing bracket.", startToken.position.line, startToken.position.character);
    setError(ERROR_MISSING_BRACKET, msg);
    return 1;
}

bool __attribute__((const))isAnnotation(Token token) {
    if (token.type != TOKEN_TYPE_KEYWORD) {
        return false;
    }
    return token.keyword == KEYWORD_BRACKET_SQUARE_OPEN;
}

//
// Keyword properties with fallthrough, partial switches
//

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wswitch"

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
    if (token.type == TOKEN_TYPE_TAKEN) {
        return false;
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


usize parseTopLevel(Tokens tokens, usize index, usize skippedTokens, AstNode **out) {
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

        case KEYWORD_STRUCT:

        case KEYWORD_TYPE:

        case KEYWORD_UNION:

        default:
            setError(ERROR_UNREACHABLE, "Unreachable");
            return 0;
    }
}

#pragma GCC diagnostic pop

AstNode *parseFile(const char *filepath) {
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

        if (isAnnotation(tokens.data[i])) {
            usize length = annotationLength(tokens, i);
            if (error()) {
                goto errorCleanup;
            }
            skippedTokens += length;
            i += length;
        }
        
        if (isTopLevelToken(tokens.data[i])) {
            AstNode *topNode = NULL;
            usize length = parseTopLevel(tokens, i, skippedTokens, &topNode);
            if (error()) {
                goto errorCleanup;
            }
            skippedTokens = 0;
            i += length;
            addAstNodes(&root->children, topNode);
        }
    }

    deleteTokens(&tokens);
    return root;

errorCleanup:
    deleteNode(root);
    deleteTokens(&tokens);
    return NULL;
}
#include "lexer.h"
#include <string.h>
#include <stdlib.h>
#include "../utils/error.h"


Keyword getKeyword(const char *line) {
    usize length = strlen(line);
    if (length == 0) {
        return NOT_KEYWORD;
    }
    if (length >= 9) {
        if (strncmp("implement", line, 9) == 0) {
            return KEYWORD_IMPLEMENT;
        }
        if (strncmp("satisfies", line, 9) == 0) {
            return KEYWORD_SATISFIES;
        }
        if (strncmp("interface", line, 9) == 0) {
            return KEYWORD_INTERFACE;
        }
        if (strncmp("namespace", line, 9) == 0) {
            return KEYWORD_NAMESPACE;
        }
    }
    if (length >= 8) {
        if (strncmp("unsigned", line, 8) == 0) {
            return KEYWORD_UNSIGNED;
        }
        if (strncmp("continue", line, 8) == 0) {
            return KEYWORD_CONTINUE;
        }
    }
    if (length >= 7) {
        if (strncmp("default", line, 7) == 0) {
            return KEYWORD_DEFAULT;
        }
        if (strncmp("dynamic", line, 7) == 0) {
            return KEYWORD_DYNAMIC;
        }
    }
    if (length >= 6) {
        if (strncmp("__FILE", line, 6) == 0) {
            return KEYWORD_E__FILE;
        }
        if (strncmp("__LINE", line, 6) == 0) {
            return KEYWORD_E__LINE;
        }
        if (strncmp("staged", line, 6) == 0) {
            return KEYWORD_STAGED;
        }
        if (strncmp("static", line, 6) == 0) {
            return KEYWORD_STATIC;
        }
        if (strncmp("return", line, 6) == 0) {
            return KEYWORD_RETURN;
        }
        if (strncmp("switch", line, 6) == 0) {
            return KEYWORD_SWITCH;
        }
        if (strncmp("inline", line, 6) == 0) {
            return KEYWORD_INLINE;
        }
        if (strncmp("extern", line, 6) == 0) {
            return KEYWORD_EXTERN;
        }
        if (strncmp("unsafe", line, 6) == 0) {
            return KEYWORD_UNSAFE;
        }
        if (strncmp("struct", line, 6) == 0) {
            return KEYWORD_STRUCT;
        }
        if (strncmp("public", line, 6) == 0) {
            return KEYWORD_PUBLIC;
        }
        if (strncmp("export", line, 6) == 0) {
            return KEYWORD_EXPORT;
        }
        if (strncmp("import", line, 6) == 0) {
            return KEYWORD_IMPORT;
        }
    }
    if (length >= 5) {
        if (strncmp("where", line, 5) == 0) {
            return KEYWORD_WHERE;
        }
        if (strncmp("yield", line, 5) == 0) {
            return KEYWORD_YIELD;
        }
        if (strncmp("union", line, 5) == 0) {
            return KEYWORD_UNION;
        }
        if (strncmp("while", line, 5) == 0) {
            return KEYWORD_WHILE;
        }
        if (strncmp("break", line, 5) == 0) {
            return KEYWORD_BREAK;
        }
        if (strncmp("const", line, 5) == 0) {
            return KEYWORD_CONST;
        }
        if (strncmp("local", line, 5) == 0) {
            return KEYWORD_LOCAL;
        }
    }
    if (length >= 4) {
        if (strncmp("self", line, 4) == 0) {
            return KEYWORD_SELF;
        }
        if (strncmp("with", line, 4) == 0) {
            return KEYWORD_WITH;
        }
        if (strncmp("enum", line, 4) == 0) {
            return KEYWORD_ENUM;
        }
        if (strncmp("else", line, 4) == 0) {
            return KEYWORD_ELSE;
        }
        if (strncmp("type", line, 4) == 0) {
            return KEYWORD_TYPE;
        }
    }
    if (length >= 3) {
        if (strncmp("ref", line, 3) == 0) {
            return KEYWORD_REF;
        }
        if (strncmp("not", line, 3) == 0) {
            return KEYWORD_NOT;
        }
        if (strncmp("for", line, 3) == 0) {
            return KEYWORD_FOR;
        }
        if (strncmp("var", line, 3) == 0) {
            return KEYWORD_VAR;
        }
        if (strncmp("mut", line, 3) == 0) {
            return KEYWORD_MUT;
        }
    }
    if (length >= 2) {
        if (strncmp("is", line, 2) == 0) {
            return KEYWORD_IS;
        }
        if (strncmp("in", line, 2) == 0) {
            return KEYWORD_IN;
        }
        if (strncmp("if", line, 2) == 0) {
            return KEYWORD_IF;
        }
        if (strncmp("fn", line, 2) == 0) {
            return KEYWORD_FN;
        }
        if (strncmp("as", line, 2) == 0) {
            return KEYWORD_AS;
        }
    }
    switch (line[0]) {
        case '@':
            return KEYWORD_AT;
        case '^':
            return KEYWORD_CARET;
        case '#':
            return KEYWORD_HASHTAG;
        case '~':
            return KEYWORD_TILDE;
        case '.':
            return KEYWORD_PERIOD;
        case ';':
            return KEYWORD_COLON_SEMI;
        case ':':
            return KEYWORD_COLON;
        case ',':
            return KEYWORD_COMMA;
        case '_':
            return KEYWORD_UNDERSCORE;
        case '-':
            return KEYWORD_MINUS;
        case '+':
            return KEYWORD_PLUS;
        case '*':
            return KEYWORD_STAR;
        case '=':
            return KEYWORD_EQUALS;
        case '&':
            return KEYWORD_AND;
        case '%':
            return KEYWORD_PERCENT;
        case '\\':
            return KEYWORD_SLASH_BACKWARD;
        case '/':
            return KEYWORD_SLASH;
        case '|':
            return KEYWORD_PIPE;
        case '?':
            return KEYWORD_MARK_QUESTION;
        case '!':
            return KEYWORD_MARK_EXCLAMATION;
        case '"':
            return KEYWORD_QUOTE_DOUBLE;
        case '\'':
            return KEYWORD_QUOTE;
        case '>':
            return KEYWORD_BRACKET_DIAMOND_CLOSE;
        case '<':
            return KEYWORD_BRACKET_DIAMOND_OPEN;
        case ']':
            return KEYWORD_BRACKET_SQUARE_CLOSE;
        case '[':
            return KEYWORD_BRACKET_SQUARE_OPEN;
        case '}':
            return KEYWORD_BRACKET_CURLY_CLOSE;
        case '{':
            return KEYWORD_BRACKET_CURLY_OPEN;
        case ')':
            return KEYWORD_BRACKET_ROUND_CLOSE;
        case '(':
            return KEYWORD_BRACKET_ROUND_OPEN;
        default:
            return NOT_KEYWORD;
    }
}

#define BUFFER_SIZE 500

    char *readLine(FILE *source) {
        char buffer[BUFFER_SIZE];
        char *result = NULL;
        usize resultLength = 0;
        usize bufferLength = 0;
        fscanf(source, " ");
        do {
            if (fgets(buffer, BUFFER_SIZE, source) == NULL) {
                if (feof(source)) {
                    return result;
                }
                char msg[100];
                snprintf(msg, 100, "Read error occured: %d", ferror(source));
                setError(ERROR_FILE_READ, msg);
                free(result);
                return NULL;
            }
            bufferLength = strlen(buffer);
            result = realloc(result, resultLength + bufferLength + 1);
            strcpy(result + resultLength, buffer);
            resultLength += bufferLength;
        } while (bufferLength == BUFFER_SIZE - 1 && buffer[BUFFER_SIZE - 2] != '\n');
        return result;
    }

    Tokenizer initTokenizer(FILE *source) {
        Tokenizer tokenizer;
        tokenizer.source = source;
        tokenizer.line = NULL;
        tokenizer.position = (SourcePosition) { .line = -1, .character = 0 };
        return tokenizer;
    }


    bool moveLine(Tokenizer *tokenizer) {
            char *line = readLine(tokenizer->source);
            if (line == NULL) {
                return false;
            }
            tokenizer->line = line;
            tokenizer->position.line++;
            return true;
    }

    bool tryReadToken(Tokenizer *tokenizer, Token *out) {
        if (tokenizer->line == NULL) {  // first line
            if (!moveLine(tokenizer)) {
                return false;
            }
        }
        // todo: change to do this before every keyword find attempt.
        if (strlen(tokenizer->line) >= 2) {
            while (tokenizer->line[0] == '/' && tokenizer->line[1] == '/') {
                if (!moveLine(tokenizer)) {
                    return false;
                }
            }

        }
        return true;
    }
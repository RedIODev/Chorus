#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"


typedef struct {
    char *character;
} Token;

typedef struct {
    usize capacity;
    usize length;
    FILE *source;
    char buffer[MAX_TOKEN_LENGTH];
} TokenIterator;


void tmpTokenPrint(Token token) {
    if (token.character == NULL) {
        printf("Token: {}");
        return;
    }
    printf("Token:%.*s\n", MAX_TOKEN_LENGTH, token.character);
}

TokenIterator createIterator(FILE *source) {
    TokenIterator iter;
    iter.length = 0;
    iter.source = source;
    return iter;
}

//check for off by 1 errors
// memory error stack override
Token iteratorRead(TokenIterator *iter) {
    usize bytesToRead = MAX_TOKEN_LENGTH - iter->length;
    usize read = fread(iter->buffer + iter->length, 1, bytesToRead, iter->source);
    if (read == 0) {
        Token t = { 0 };
        return t;
    }

    for (usize i = 0; i < read; i++) {
        if (isspace(iter->buffer[i])) {
            Token result;
            result.character = malloc(i);
            memcpy(result.character, iter->buffer, read-1);
            result.character[i] = '\0';

            memmove(iter->buffer, iter->buffer + i + 1, read - i - 1);
            iter->length = read - i - 1;

            return result;
        }
    }

    if (read < bytesToRead) {
        Token result;
        result.character = malloc(read + iter->length + 1);
        memcpy(result.character, iter->buffer, read + iter->length);
        result.character[read + iter->length + 1] = '\0';
        return result;
    }
    
    exit(EXIT_FAILURE); //lazy rework
}


AstNode *parseFile(const char *filepath) {
    FILE *file = fopen(filepath, "r");
    if (file == NULL) {
        char *msg;
        STRCPY(msg, strerror(errno));
        setError(ERROR_PARSE_FILE_OPEN, msg);
        return NULL;
    }

    AstNode *root = createFileNode();
    FileRootNode *fileRootData = GET_NODE_DATA(FileRootNode, root);
    STRCPY(fileRootData->path, filepath);
    TokenIterator iter = createIterator(file);
    for (usize i = 0; i < 4; i++)
    {
        Token token = iteratorRead(&iter);
        tmpTokenPrint(token);
        free(token.character);
    }
    

    fclose(file);
    errorClear();
    return root;
}


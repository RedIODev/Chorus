#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"
#include "lexer.h"

typedef struct {
    Token *data;
    usize size;
    usize capacity;
} Tokens;

#define TOKENS_START_SIZE 10

void tokensAddToken(Tokens *tokens,Token token) {
    if (tokens->data == NULL) {
        tokens->data = malloc(sizeof(Token) * TOKENS_START_SIZE);
        tokens->capacity = TOKENS_START_SIZE;
        tokens->size = 0;
    }
    if (tokens->capacity <= tokens->size) { //grow array
        tokens->capacity += (tokens->capacity / 2);
        //printf("Tokens capacity: %ld, size:%ld\n", tokens->capacity, tokens->size);
        tokens->data = realloc(tokens->data, sizeof(Token) * tokens->capacity);
    }
    tokens->data[tokens->size++] = token;
    
}

void deleteTokens(Tokens *tokens) {
    for (usize i = 0; i < tokens->size; i++) {
        Token token = tokens->data[i];
        if (token.type == TOKEN_TYPE_IDENTIFIER) {
            free(token.identifier.name);
        }
    }
    
    tokens->capacity = 0;
    tokens->size = 0;
    free(tokens->data);
    tokens->data = NULL;
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
        tokensAddToken(&tokens, token);
    }

    deleteTokenizer(&tokensizer);
    fclose(file);
    return tokens;
}

AstNode *parseFile(const char *filepath) {
    AstNode *root = createNode(NODE_TYPE_FILE_ROOT);
    FileRootNode *fileRootData = GET_NODE_DATA(FileRootNode, root);
    STRCPY(fileRootData->path, filepath);
   
    Tokens tokens = readTokens(filepath);
    if (error()) {
        deleteNode(root);
        deleteTokens(&tokens);
        return NULL;
    }

    //parse Ast.

    deleteTokens(&tokens);
    return root;
}
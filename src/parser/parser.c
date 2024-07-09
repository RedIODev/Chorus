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

Tokens createTokens() {
    Tokens tokens;
    tokens.capacity = 0;
    tokens.size = 0;
    tokens.data = NULL;
    return tokens;
}

void tokensAddToken(Tokens *tokens,Token token) {
    if (tokens->data == NULL) {
        tokens->data = malloc(sizeof(Token) * TOKENS_START_SIZE);
        tokens->capacity = TOKENS_START_SIZE;
        tokens->size = 0;
    }
    if (tokens->capacity <= tokens->size) { //grow array
        tokens->capacity += (tokens->capacity / 2);
        printf("Tokens capacity: %ld, size:%ld\n", tokens->capacity, tokens->size);
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

AstNode *parseFile(const char *filepath) {
    FILE *file = fopen(filepath, "r");
    if (file == NULL) {
        char *msg;
        STRCPY(msg, strerror(errno));
        setError(ERROR_FILE_OPEN, msg);
        return NULL;
    }

    AstNode *root = createFileNode();
    FileRootNode *fileRootData = GET_NODE_DATA(FileRootNode, root);
    STRCPY(fileRootData->path, filepath);
   
    Tokenizer tokenizer = createTokenizer(file);
    Tokens tokens = createTokens();
    Token token;
    while (tryReadToken(&tokenizer, &token)) {
        switch (token.type) {
            case TOKEN_TYPE_KEYWORD:
                printf("Token { x:%d, y:%d, keyword: %d}\n", token.position.line, token.position.character, token.keyword);
                break;
            case TOKEN_TYPE_IDENTIFIER:
                printf("Token { x:%d, y:%d, identifier: %s}\n", token.position.line, token.position.character, token.identifier.name);
                break;
        }
        tokensAddToken(&tokens, token);
    }
    if (error()) {
        printf(errorMessage());
    }
    deleteTokenizer(&tokenizer);
    deleteTokens(&tokens);
    fclose(file);
    errorClear();
    return root;
}
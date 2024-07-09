#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"
#include "lexer.h"


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
    bool success = true;
    while (success) {
        Token token;
        success = tryReadToken(&tokenizer, &token);

        
        switch (token.type) {
            case TOKEN_TYPE_KEYWORD:
                printf("Token { x:%d, y:%d, keyword: %d}\n", token.position.line, token.position.character, token.keyword);
                break;
            case TOKEN_TYPE_IDENTIFIER:
                printf("Token { x:%d, y:%d, identifier: %s}\n", token.position.line, token.position.character, token.identifier.name);
                break;
        }
        if (token.type == TOKEN_TYPE_IDENTIFIER) {
            free(token.identifier.name);
        }
    }
    if (error()) {
        printf(errorMessage());
    }

    fclose(file);
    errorClear();
    return root;
}
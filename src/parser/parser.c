#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"


typedef enum {
    TOKEN_TYPE_IDENTIFIER,
    TOKEN_TYPE_KEYWORD,
} TokenType;

typedef struct {
    u32 line;
    u32 character;
} SourcePosition;

typedef enum {
    
} Keyword;

typedef struct {
    char *name;
} Identifier;

typedef struct {
    SourcePosition position;
    TokenType type;
    union {
        Identifier identifier; 
        Keyword keyword;
    }
} Token;

void tmpTokenPrint(Token token) {
    
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
   

    

    fclose(file);
    errorClear();
    return root;
}


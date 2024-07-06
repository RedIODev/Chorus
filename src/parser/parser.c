#include "parser.h"
#include "../utils/error.h"
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>
#include "../utils/string.h"


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
   

    

    fclose(file);
    errorClear();
    return root;
}


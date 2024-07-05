#include "parser/parser.h"
#include <stdlib.h>
#include <stdio.h>
#include "utils/error.h"

int main(int argc, char const *argv[])
{
    if (argc < 2) {
        return -1;
    }
    AstNode *root = parseFile(argv[1]);
    if (error()) {
        printf("%s\n", errorMessage());
    }

    char buffer[500];
    nodeToString(buffer, 500, root);
    printf(buffer);

    destroyNode(root);
}
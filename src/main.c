#include "parser/parser.h"
#include <stdlib.h>
#include <stdio.h>
#include "utils/error.h"

int main(int argc, char const *argv[])
{
    if (argc < 2) {
        return -1;
    }
    ParseResult result = parseFile(argv[1]);
    if (error()) {
        printf("%s\n", errorMessage());
        return -1;
    }

    char buffer[500];
    nodeToString(buffer, 500, result.root);
    printf(buffer);

    // 1073741816
    deleteParserResult(result);
}
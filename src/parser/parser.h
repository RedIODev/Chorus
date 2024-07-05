#ifndef PARSER_H
#define PARSER_H
#include "../utils/primitive.h"
#include "ast.h"

#define MAX_TOKEN_LENGTH 255

AstNode *parseFile(const char *);




#endif
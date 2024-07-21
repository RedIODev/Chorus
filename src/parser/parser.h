#ifndef PARSER_H
#define PARSER_H
#include "../utils/primitive.h"
#include "ast.h"
#include "lexer.h"

void deleteToken(Token token) {
    if (token.type == TOKEN_TYPE_IDENTIFIER) {
        free(token.identifier.name);
    }
}

CREATE_VECTOR_TYPE(Tokens, Token, deleteToken)

typedef struct {
    Tokens tokens;
    AstNode *root;
} ParseResult;


ParseResult parseFile(const char *);

void deleteParserResult(ParseResult result);


#endif
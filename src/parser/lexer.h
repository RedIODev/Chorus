#ifndef PARSER_LEXER_H
#define PARSER_LEXER_H
#include "../utils/primitive.h"
#include <stdio.h>

typedef enum {
    TOKEN_TYPE_IDENTIFIER,
    TOKEN_TYPE_KEYWORD,
} TokenType;

typedef struct {
    u32 line;
    usize character;
} SourcePosition;

typedef enum {
    //INSERT NEW KEYWORD AT FRONT OF BLOCK!!
    NOT_KEYWORD,
    //symbols
    KEYWORD_BRACKET_ROUND_OPEN,
    KEYWORD_BRACKET_ROUND_CLOSE,
    KEYWORD_BRACKET_CURLY_OPEN,
    KEYWORD_BRACKET_CURLY_CLOSE,
    KEYWORD_BRACKET_SQUARE_OPEN,
    KEYWORD_BRACKET_SQUARE_CLOSE,
    KEYWORD_BRACKET_DIAMOND_OPEN,
    KEYWORD_BRACKET_DIAMOND_CLOSE,
    KEYWORD_QUOTE,
    KEYWORD_QUOTE_DOUBLE,
    KEYWORD_MARK_EXCLAMATION,
    KEYWORD_MARK_QUESTION,
    KEYWORD_PIPE,
    KEYWORD_SLASH,
    KEYWORD_SLASH_BACKWARD,
    KEYWORD_PERCENT,
    KEYWORD_AND,
    KEYWORD_EQUALS,
    KEYWORD_STAR,
    KEYWORD_PLUS,
    KEYWORD_MINUS,
    KEYWORD_UNDERSCORE,
    KEYWORD_COMMA,
    KEYWORD_COLON,
    KEYWORD_COLON_SEMI,
    KEYWORD_PERIOD,
    KEYWORD_TILDE,
    KEYWORD_HASHTAG,
    KEYWORD_CARET,
    KEYWORD_AT,
    // 2 letter keywords
    KEYWORD_AS,
    KEYWORD_FN,
    KEYWORD_IF,
    KEYWORD_IN,
    KEYWORD_IS,
    // 3 letter keywords
    KEYWORD_MUT,
    KEYWORD_VAR,
    KEYWORD_FOR,
    KEYWORD_NOT,
    KEYWORD_REF,
    // 4 letter keywords
    KEYWORD_TYPE,
    KEYWORD_ELSE,
    KEYWORD_ENUM,
    KEYWORD_WITH,
    KEYWORD_SSELF,
    KEYWORD_SELF,
    // 5 letter keywords
    KEYWORD_LOCAL,
    KEYWORD_CONST,
    KEYWORD_BREAK,
    KEYWORD_WHILE,
    KEYWORD_UNION,
    KEYWORD_YIELD,
    KEYWORD_WHERE,
    // 6 letter keywords
    KEYWORD_IMPORT,
    KEYWORD_EXPORT,
    KEYWORD_PUBLIC,
    KEYWORD_STRUCT,
    KEYWORD_UNSAFE,
    KEYWORD_EXTERN,
    KEYWORD_INLINE,
    KEYWORD_SWITCH,
    KEYWORD_RETURN,
    KEYWORD_STATIC,
    KEYWORD_STAGED,
    KEYWORD_E__LINE,
    KEYWORD_E__FILE,
    // 7 letter keywords
    KEYWORD_DYNAMIC,
    KEYWORD_DEFAULT,
    // 8 letter keywords
    KEYWORD_CONTINUE,
    KEYWORD_UNSIGNED,
    // 9 letter keywords
    KEYWORD_NAMESPACE,
    KEYWORD_INTERFACE,
    KEYWORD_SATISFIES,
    KEYWORD_IMPLEMENT,
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
    };
} Token;

typedef struct {
    FILE *source;
    char *line;
    SourcePosition position;
} Tokenizer;

Tokenizer createTokenizer(FILE *);

bool tryReadToken(Tokenizer *, Token *);

void deleteTokenizer(Tokenizer *);

#endif
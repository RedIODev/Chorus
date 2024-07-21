#ifndef UTILS_ERROR_H
#define UTILS_ERROR_H
#include "primitive.h"

typedef enum {
    ERROR_NO_ERROR,
    ERROR_FILE_OPEN,
    ERROR_FILE_READ,
    ERROR_INVALID_NODE_TYPE,
    ERROR_NULL_POINTER_ARGUMENT,
    ERROR_INVALID_KEYWORD,
    ERROR_BUFFER_WRITE,
    ERROR_OUT_OF_BOUNDS,
    ERROR_BRACKET_MISSMATCH,
    ERROR_UNREACHABLE,
    ERROR_INVALID_TOKEN,
    ERROR_INCOMPLETE_STATEMENT,
} ErrorCode;

bool error(void);
ErrorCode errorCode(void);
const char *errorMessage(void);
void setError(ErrorCode, const char*);
void errorClear(void);

#endif
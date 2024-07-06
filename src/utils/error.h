#ifndef UTILS_ERROR_H
#define UTILS_ERROR_H
#include "primitive.h"

typedef enum {
    ERROR_NO_ERROR,
    ERROR_FILE_OPEN,
    ERROR_FILE_READ,
    ERROR_INVALID_NODE_TYPE,
} ErrorCode;

bool error(void);
ErrorCode errorCode(void);
const char *errorMessage(void);
void setError(ErrorCode, const char*);
void errorClear(void);

#endif
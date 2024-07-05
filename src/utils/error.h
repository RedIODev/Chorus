#ifndef UTILS_ERROR_H
#define UTILS_ERROR_H
#include "primitive.h"

bool error(void);
u8 errorCode(void);
const char *errorMessage(void);
void setError(u8, const char*);
void errorClear(void);

//
// ERROR_CODES
//

#define ERROR_NO_ERROR 0
#define ERROR_PARSE_FILE_OPEN 1
#define ERROR_INVALID_NODE_TYPE 2

#endif
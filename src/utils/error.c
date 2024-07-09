#include "error.h"
#include <stdlib.h>
#include <string.h>
#include "string.h"
_Thread_local char *tl_message = NULL;
_Thread_local ErrorCode tl_code = ERROR_NO_ERROR;

bool __attribute__((pure))error(void) {
    return tl_code != ERROR_NO_ERROR;
}
ErrorCode __attribute__((pure))errorCode(void) {
    return tl_code;
}
const char *__attribute__((pure))errorMessage(void) {
    return tl_message;
}
void setError(ErrorCode code, const char *message) {
    tl_code = code;
    if (tl_message != NULL) {
        free(tl_message);
        tl_message = NULL;
    }
    if (message == NULL) {
        return;
    }
   
    char *newMessage;
    STRCPY(newMessage, message);
    tl_message = newMessage;
}
void errorClear(void) {
    tl_code = ERROR_NO_ERROR;
    if (tl_message != NULL) {
        free(tl_message);
    }
    tl_message = NULL;
}
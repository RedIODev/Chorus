#include "error.h"
#include <stdlib.h>
String *message = NULL;

bool error() {
    return message != NULL;
}
String *errorMessage() {
    String *msg = message;
    message = NULL;
    return msg;
}
void errorSetMessage(String *msg) {
    if (message != NULL) {
        objectDelete(objectCast(msg));
    }
}
void errorClear() {
    if (message == NULL) {
        return;
    }
    objectDelete(objectCast(message));
    message = NULL;
}
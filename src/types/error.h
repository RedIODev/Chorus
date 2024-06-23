#ifndef ERROR_H
#define ERROR_H
#include "string.h"

bool error();
String *errorMessage();
void errorSetMessage(String*);
void errorClear();

#endif
#ifndef UTILS_STRING_H 
#define UTILS_STRING_H
#include "primitive.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define STRCPY(DEST, SRC) { \
const char *srcStr = SRC; \
usize len = strlen(srcStr); \
DEST = malloc(len+1); \
memcpy(DEST, srcStr, len+1); \
} 


void printStringEscaped(const char *);

#endif
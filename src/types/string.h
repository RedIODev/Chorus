#ifndef STRING_H
#define STRING_H
#include "object.h"

typedef struct String_ String;
typedef struct StringView_ {
    const String *string;
    usize start;
    usize end;
} StringView;
typedef struct StringIter_ StringIter;

String *stringNewEmpty();
String *stringFromCString(const char*);
usize stringLength(const String*);
usize stringCapacity(const String*);
StringIter stringIter(const String*);
bool stringEquals(const String*, const String*);
bool stringIsEmpty(const String*);
c32 stringGet(const String*, usize);
const char *stringCString(const String*);
void stringClear(String*);
void stringReserve(String*, usize);
void stringAddChar(String*, c32);
void stringAddString(String*, const String*);
void stringAddCString(String*, const char*);
c32 stringRemove(String*, usize);


StringView stringViewFromString(const String*);
usize stringViewLength(StringView);
StringView stringViewSubView(const String*, usize, usize);
StringIter stringViewIter(StringView);


#endif
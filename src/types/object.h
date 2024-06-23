#ifndef OBJECT_H
#define OBJECT_H
#include "primitive.h"

typedef struct Object_ Object;
typedef void (*Destructor(Object*));

Object *objectCast(void*);
bool objectIsObject(void*);
void objectDelete(Object*);
usize objectTypeId(const Object*);
void objectInit(Object*, Destructor*, usize);
void objectHeapInit(Object*, Destructor*, usize);
bool objectIsHeap(const Object*);
bool objectIsAlive(const Object*);
#endif // HEADER GUARD
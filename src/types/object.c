#include "object.h"
#include <stdlib.h>
typedef struct Object_ {
    Destructor *destructor;
    usize typeId;
    bool isHeap;
    bool isAlive;
    u8 objectByte;
} Object;

#define OBJECT_BYTE 42

void objectInitInternal(Object *object, Destructor *destructor, usize typeId, bool isHeap);

Object *objectCast(void* data) {
    if (!objectIsObject(data)) {
        return NULL;
    }
    Object *object = (Object*) data;
    return object;
}

bool objectIsObject(void *data) {
    if (data == NULL) {
        return false;
    }
    Object *object = (Object*) data;
    return object->objectByte == OBJECT_BYTE;
}
void objectDelete(Object *object) {
    if (object == NULL) {
        return;
    }
    if (object->isAlive) {
        return;
    }
    if (object->destructor != NULL) {
        object->destructor(object);
    }
    object->isAlive = false;
    if (object->isHeap) {
        free(object);
    }
}
usize objectTypeId(const Object *object) {
    if (object == NULL) {
        return 0;
    }
    return object->typeId;
}
void objectInit(Object *object, Destructor *destructor, usize typeId) {
    objectInitInternal(object, destructor, typeId, false);
}

void objectHeapInit(Object *object, Destructor *destructor, usize typeId) {
    objectInitInternal(object, destructor, typeId, true);
}
bool objectIsHeap(const Object *object) {
    if (object == NULL) {
        return false;
    }
    return object->isHeap;
}
bool objectIsAlive(const Object *object) {
    if (object == NULL) {
        return false;
    }
    return object->isAlive;
}

void objectInitInternal(Object *object, Destructor *destructor, usize typeId, bool isHeap) {
    if (object == NULL) {
        return;
    }
    object->destructor = destructor;
    object->typeId = typeId;
    object->isHeap = isHeap;
    object->isAlive = true;
}
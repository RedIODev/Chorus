#ifndef UTILS_VECTOR_H
#define UTILS_VECTOR_H

#include "primitive.h"
#include "error.h"
#include <stdlib.h>

#define VECTOR_START_SIZE 20


#define VECTOR_BOUNDS_CHECK(VECTOR, INDEX, RETURN_VALUE)                                            \
    if (VECTOR.size <= INDEX) {                                                                     \
        char msg[100];                                                                              \
        snprintf(msg, 100, "Index %ld is out of bounds for %s[%ld]", INDEX, #VECTOR, VECTOR.size);  \
        setError(ERROR_OUT_OF_BOUNDS, msg);                                                         \
        return RETURN_VALUE;                                                                        \
    }

#define CREATE_VECTOR_TYPE(NAME, TARGET, TARGET_DESTRUCTOR)                     \
typedef struct {                                                                \
    TARGET *data;                                                               \
    usize capacity;                                                             \
    usize size;                                                                 \
} NAME;                                                                         \
                                                                                \
void delete##NAME(NAME *vector) {                                               \
    if (TARGET_DESTRUCTOR != NULL) {                                            \
        for (usize i = 0; i < vector->size; i++) {                              \
            void (*ptr) (TARGET) = TARGET_DESTRUCTOR;                           \
            ptr(vector->data[i]);                                               \
        }                                                                       \
    }                                                                           \
    vector->capacity = 0;                                                       \
    vector->size = 0;                                                           \
    free(vector->data);                                                         \
    vector->data = NULL;                                                        \
}                                                                               \
                                                                                \
void add##NAME(NAME *vector, TARGET item) {                                     \
    if (vector->data == NULL) {                                                 \
        vector->data = malloc(sizeof(TARGET) * VECTOR_START_SIZE);              \
        vector->capacity = VECTOR_START_SIZE;                                   \
        vector->size = 0;                                                       \
    }                                                                           \
    if (vector->capacity <= vector->size) {                                     \
        vector->capacity += (vector->capacity / 2);                             \
        vector->data = realloc(vector->data, sizeof(TARGET) * vector->capacity);\
    }                                                                           \
    vector->data[vector->size++] = item;                                        \
}                                                                               \
                                                                                \
TARGET pop##NAME(NAME *stack) {                                                 \
    if (stack->size <= 0) {                                                     \
        setError(ERROR_OUT_OF_BOUNDS, "Stack is empty");                        \
        return (TARGET) {0};                                                    \
    }                                                                           \
    return stack->data[stack->size--];                                          \
}





#endif

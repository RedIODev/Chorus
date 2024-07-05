#ifndef PARSER_AST_H
#define PARSER_AST_H
#include "../utils/primitive.h"
#include <stdlib.h>

//
// Types
//

typedef enum {
    NODE_TYPE_FILE_ROOT,
    NODE_TYPE_NAMESPACE,
} NodeType;

typedef enum {
    ACCESS_MODIFIER_DEFAULT_PRIVATE,
    ACCESS_MODIFIER_PUBLIC,
    ACCESS_MODIFIER_LOCAL,

} AccessModifier;


typedef struct AstNode {
    struct AstNode *parent;
    struct AstNode *children;
    usize children_n;
    NodeType type;
    u8 data[];
} AstNode;

typedef struct {
    char *path;
} FileRootNode;

typedef struct {
    char *name;
    AccessModifier accessModifier;
} NamespaceNode;

//
// Functions
//

void destroyNode(AstNode *);
usize nodeToString(char *, usize, const AstNode *);

AstNode *createFileNode(void);
AstNode *createNamespaceNode(void);

//
// Macros
//

#define MAKE_NODE(TYPE) malloc(sizeof(AstNode) + sizeof(TYPE))
#define GET_NODE_DATA(TYPE, node) (TYPE*) node->data

#endif
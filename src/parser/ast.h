#ifndef PARSER_AST_H
#define PARSER_AST_H
#include "../utils/primitive.h"
#include "../utils/vector.h"
#include <stdlib.h>

//
// Types
//

typedef enum {
    NODE_TYPE_FILE_ROOT,
    NODE_TYPE_NAMESPACE,
    NODE_TYPE_IMPORT,
} NodeType;

void handleInvalidNodeType(NodeType type);

typedef enum {
    ACCESS_MODIFIER_DEFAULT_PRIVATE,
    ACCESS_MODIFIER_PUBLIC,
    ACCESS_MODIFIER_LOCAL,

} AccessModifier;

// typedef struct {
//     char *name;
//     char **arguments;
//     usize arguments_n;
// } Attribute;

typedef struct AstNode_s AstNode;

//
// Ast Functions
//
AstNode *createNode(NodeType);
void deleteNode(AstNode *);
u32 nodeToString(char *, usize, const AstNode *);

CREATE_VECTOR_TYPE(AstNodes, AstNode*, deleteNode)

typedef struct AstNode_s {
    AstNode *parent;
    AstNodes children;
    NodeType type;
    u8 data[];
} AstNode;

typedef struct {
    char *path;
} FileRootNode;

typedef struct {
    char *name;
    AccessModifier accessModifier;
    // Attribute *attributes;
    // usize attributes_n;
} NamespaceNode;

typedef struct {
    // Attribute *attributes;
    // usize attributes_n;
    char **namespacePath;
    usize namespacePath_n;
} ImportNode;

//
// Macros
//

#define MAKE_NODE(TYPE) malloc(sizeof(AstNode) + sizeof(TYPE))
#define GET_NODE_DATA(TYPE, node) (TYPE*) node->data

#endif
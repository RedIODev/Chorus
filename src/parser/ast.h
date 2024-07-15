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
    NODE_TYPE_IMPORT,
} NodeType;

typedef enum {
    ACCESS_MODIFIER_DEFAULT_PRIVATE,
    ACCESS_MODIFIER_PUBLIC,
    ACCESS_MODIFIER_LOCAL,

} AccessModifier;

typedef struct {
    char *name;
    char **arguments;
    usize arguments_n;
} Attribute;

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
    Attribute *attributes;
    usize attributes_n;
} NamespaceNode;

typedef struct {
    Attribute *attributes;
    usize attributes_n;
    char **namespacePath;
    usize namespacePath_n;
} ImportNode;

//
// Functions
//

AstNode *createNode(NodeType type);
void deleteNode(AstNode *);
u32 nodeToString(char *, usize, const AstNode *);


//
// Macros
//

#define MAKE_NODE(TYPE) malloc(sizeof(AstNode) + sizeof(TYPE))
#define GET_NODE_DATA(TYPE, node) (TYPE*) node->data

#endif
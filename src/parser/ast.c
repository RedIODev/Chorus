#include "ast.h"
#include <string.h>
#include <stdio.h>
#include "../utils/error.h"


const char *nodeTypeToString(NodeType type) {
    switch (type) {
        case NODE_TYPE_FILE_ROOT:
            return "FileRoot";
        case NODE_TYPE_NAMESPACE:
            return "Namespace";
        default:
            return "Invalid";
    }
}

const char *accessModifierToString(AccessModifier modifier) {
    switch (modifier) {
        case ACCESS_MODIFIER_DEFAULT_PRIVATE:
            return "private(default)";
        case ACCESS_MODIFIER_LOCAL:
            return "local";
        case ACCESS_MODIFIER_PUBLIC:
            return "public";
        default:
            return "Invalid";
    }
}

void handleInvalidNodeType(NodeType type) {
    char msg[30];
    snprintf(msg, 30, "Invalid Node type: %d", type);
    setError(ERROR_INVALID_NODE_TYPE, msg);
}

//
// Node Destruction
//

void destroyFileNode(AstNode *node) {
    FileRootNode *fileData = GET_NODE_DATA(FileRootNode, node);
    free(fileData->path);
}

void destroyNamespaceNode(AstNode *node) {
    NamespaceNode *namespaceData = GET_NODE_DATA(NamespaceNode, node);
    free(namespaceData->name);
}

void destroyNode(AstNode *node) {
    switch(node->type) {
        case NODE_TYPE_FILE_ROOT:
            destroyFileNode(node);
            break;
        case NODE_TYPE_NAMESPACE:
            destroyNamespaceNode(node);
        default:
            handleInvalidNodeType(node->type);
            return;
    }
    free(node);
}

//
// Node ToString
//

usize fileNodeToString(char *buffer, usize length, const AstNode *node) {
    FileRootNode *fileData = GET_NODE_DATA(FileRootNode, node);
    return snprintf(buffer, length, "Filepath: \"%s\" }", fileData->path);
}

usize namespaceNodeToString(char *buffer, usize length, const AstNode *node) {
    NamespaceNode *namespaceData = GET_NODE_DATA(NamespaceNode, node);
    return snprintf(buffer, length, "Name: %s, AccessModifier: %s", namespaceData->name, accessModifierToString(namespaceData->accessModifier));
}

usize nodeToString(char *buffer, usize length, const AstNode *node) {
    const char *parentType = "NULL";
    if (node->parent != NULL) {
        parentType = nodeTypeToString(node->parent->type);
    }

    usize end = snprintf(buffer, length, "%sNode { Parent: %s, Children: [%ld], ", nodeTypeToString(node->type), parentType, node->children_n);

    switch(node->type) {
        case NODE_TYPE_FILE_ROOT:
            end += fileNodeToString(buffer + end, length - end, node);
            break;
        default:
            handleInvalidNodeType(node->type);
            return end;
    }
    return end;
}

AstNode *createFileNode(void) {
    AstNode *fileNode = MAKE_NODE(FileRootNode);
    fileNode->type = NODE_TYPE_FILE_ROOT;
    return fileNode;
}
#include "ast.h"
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include "../utils/error.h"


const char * __attribute__((const))nodeTypeToString(NodeType type) {
    switch (type) {
        case NODE_TYPE_FILE_ROOT:
            return "FileRoot";
        case NODE_TYPE_NAMESPACE:
            return "Namespace";
        default:
            return "Invalid";
    }
}

const char * __attribute__((const))accessModifierToString(AccessModifier modifier) {
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

bool handleBufferWrite(i32 errorCode) {
    if (errorCode < 0) {
        setError(ERROR_BUFFER_WRITE, strerror(errno));
        return true;
    }
    return false;
}

//
// Node Destruction
//

void deleteFileNode(AstNode *node) {
    FileRootNode *fileData = GET_NODE_DATA(FileRootNode, node);
    free(fileData->path);
}

void deleteNamespaceNode(AstNode *node) {
    NamespaceNode *namespaceData = GET_NODE_DATA(NamespaceNode, node);
    free(namespaceData->name);
}

void deleteNode(AstNode *node) {
    switch(node->type) {
        case NODE_TYPE_FILE_ROOT:
            deleteFileNode(node);
            break;
        case NODE_TYPE_NAMESPACE:
            deleteNamespaceNode(node);
            break;
        default:
            handleInvalidNodeType(node->type);
            return;
    }
    free(node);
}

//
// Node ToString
//

i32 fileNodeToString(char *buffer, usize length, const AstNode *node) {
    FileRootNode *fileData = GET_NODE_DATA(FileRootNode, node);
    return snprintf(buffer, length, "Filepath: \"%s\" }", fileData->path);
}

i32 namespaceNodeToString(char *buffer, usize length, const AstNode *node) {
    NamespaceNode *namespaceData = GET_NODE_DATA(NamespaceNode, node);
    return snprintf(buffer, length, "Name: %s, AccessModifier: %s", namespaceData->name, accessModifierToString(namespaceData->accessModifier));
}

u32 nodeToString(char *buffer, usize length, const AstNode *node) {
    const char *parentType = "NULL";
    if (node->parent != NULL) {
        parentType = nodeTypeToString(node->parent->type);
    }

    i32 errorResult = snprintf(buffer, length, "%sNode { Parent: %s, Children: [%ld], ", nodeTypeToString(node->type), parentType, node->children_n);

    if (handleBufferWrite(errorResult)) {
        return 0;
    }
    u32 messageEnd = (u32)errorResult;

    switch(node->type) {
        case NODE_TYPE_FILE_ROOT:
            errorResult = fileNodeToString(buffer + messageEnd, length - messageEnd, node);
            if (handleBufferWrite(errorResult)) {
                return 0;
            }
            messageEnd += (u32)errorResult;
            break;
        case NODE_TYPE_NAMESPACE:
            errorResult = namespaceNodeToString(buffer + messageEnd, length - messageEnd, node);
            if (handleBufferWrite(errorResult)) {
                return 0;
            }
            messageEnd += (u32)errorResult;
            break;
        default:
            handleInvalidNodeType(node->type);
            return messageEnd;
    }
    return messageEnd;
}

AstNode *createFileNode(void) {
    AstNode *fileNode = MAKE_NODE(FileRootNode);
    fileNode->type = NODE_TYPE_FILE_ROOT;
    return fileNode;
}
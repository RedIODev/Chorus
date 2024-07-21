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
        case NODE_TYPE_IMPORT:
            return "Import";
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

bool handleBufferWrite(i64 errorCode) {
    if (errorCode < 0) {
        setError(ERROR_BUFFER_WRITE, strerror(errno));
        return true;
    }
    return false;
}

// void deleteAttributes(Attribute *attributes, usize attribute_n) {
//     for (usize i = 0; i < attribute_n; i++) {
//         free(attributes[i].name);
//         for (usize j = 0; j < attributes[i].arguments_n; i++) {
//             free(attributes[i].arguments[j]);
//         }
//         free(attributes[i].arguments);
//     }
//     free(attributes);
// }

//
// Node Destruction
//

void deleteFileNode(AstNode *node) {
    FileRootNode *data = GET_NODE_DATA(FileRootNode, node);
    free(data->path);
    data->path = NULL;
}

void deleteNamespaceNode(AstNode *node) {
    NamespaceNode *data = GET_NODE_DATA(NamespaceNode, node);
    free(data->name);
    data->name = NULL;
    // deleteAttributes(data->attributes, data->attributes_n);
    // data->attributes = NULL;
}

void deleteImportNode(AstNode *node) {
    ImportNode *data = GET_NODE_DATA(ImportNode, node);
    //deleteAttributes(data->attributes, data->attributes_n);
    // data->attributes = NULL;
    for (usize i = 0; i < data->namespacePath_n; i++) {
        free(data->namespacePath[i]);
    }
    free(data->namespacePath);
    data->namespacePath = NULL;
}

void deleteNode(AstNode *node) {
    switch(node->type) {
        case NODE_TYPE_FILE_ROOT:
            deleteFileNode(node);
            break;
        case NODE_TYPE_NAMESPACE:
            deleteNamespaceNode(node);
            break;
        case NODE_TYPE_IMPORT:
            deleteImportNode(node);
            break;
        default:
            handleInvalidNodeType(node->type);
            return;
    }
    for (usize i = 0; i < node->children.size; i++) {
        deleteNode(node->children.data[i]);
    }
    
    free(node);
}

//
// Node ToString
//

i32 fileNodeToString(char *buffer, usize length, const AstNode *node) {
    FileRootNode *data = GET_NODE_DATA(FileRootNode, node);
    return snprintf(buffer, length, "Filepath: \"%s\" }", data->path);
}

i32 namespaceNodeToString(char *buffer, usize length, const AstNode *node) {
    NamespaceNode *data = GET_NODE_DATA(NamespaceNode, node);
    return snprintf(buffer, length, "Name: %s, AccessModifier: %s", data->name, accessModifierToString(data->accessModifier));
}

i64 importNodeToString(char *buffer, usize length, const AstNode *node) {
    ImportNode *data = GET_NODE_DATA(ImportNode, node);
    u32 writtenBytes = 0;
    i32 error = 0;
    error = snprintf(buffer, length, "Path: ");
    if (error < 0) {
        return error;
    }
    writtenBytes += (u32)error;
    for (usize i = 0; i < data->namespacePath_n-1; i++) {
        if (i >= data->namespacePath_n -1) {
            break;
        }
        error = snprintf(buffer+writtenBytes, length - writtenBytes, "%s::", data->namespacePath[i]);
        if (error < 0) {
            return error;
        }
        writtenBytes += (u32)error;
    }
    error = snprintf(buffer+writtenBytes, length - writtenBytes, "%s", data->namespacePath[data->namespacePath_n-1]);
    if (error < 0) {
        return error;
    }
    writtenBytes += (u32)error;
    return writtenBytes;
}

u32 nodeToString(char *buffer, usize length, const AstNode *node) {
    const char *parentType = "NULL";
    if (node->parent != NULL) {
        parentType = nodeTypeToString(node->parent->type);
    }

    i64 errorResult = snprintf(buffer, length, "%sNode { Parent: %s, Children: [%ld], ", nodeTypeToString(node->type), parentType, node->children.size);

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
        case NODE_TYPE_IMPORT:
            errorResult = importNodeToString(buffer + messageEnd, length - messageEnd, node);
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

AstNode *createNode(NodeType type) {
    AstNode *node;
    switch (type)
    {
    case NODE_TYPE_FILE_ROOT:
        node = MAKE_NODE(FileRootNode);
        break;
    case NODE_TYPE_NAMESPACE:
        node = MAKE_NODE(NamespaceNode);
        break;
    case NODE_TYPE_IMPORT:
        node = MAKE_NODE(ImportNode);
        break;
    default:
        handleInvalidNodeType(type);
        return NULL;
    }
    node->type = type;
    return node;
}
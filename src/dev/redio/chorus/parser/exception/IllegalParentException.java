package dev.redio.chorus.parser.exception;

import dev.redio.chorus.parser.node.Node;

public class IllegalParentException extends IllegalNodeException {
    
    private Node parent;

    public IllegalParentException(Node parent) {
        super(messageBuilder(parent));
        this.parent = parent;
    }

    public Node parent() {
        return parent;
    }

    private static String messageBuilder(Node parent) {
        if (parent == null)
            return "Parent was null";
        return "Parent was " + parent.getClass().getSimpleName();
    }
}

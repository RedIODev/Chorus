package dev.redio.chorus.parser.exception;

import dev.redio.chorus.parser.node.Node;

public class IllegalParentException extends IllegalNodeException {
    
    private Node parent;
    private Node child;

    public IllegalParentException(Node parent, Node child) {
        super(messageBuilder(parent, child));
        this.parent = parent;
        this.child = child;
    }

    public Node parent() {
        return parent;
    }

    public Node child() {
        return child;
    }

    private static String messageBuilder(Node parent, Node child) {
        if (parent == null)
            return "Parent for " + child.getClass().getName() + " was null";
        return "Parent " + parent.getClass().getSimpleName() + "is not allowed for Child " + child.getClass().getSimpleName();
    }
}

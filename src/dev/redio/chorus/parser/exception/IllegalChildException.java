package dev.redio.chorus.parser.exception;

import dev.redio.chorus.parser.node.Node;

public class IllegalChildException extends IllegalNodeException {
     private Node child;

    public IllegalChildException(Node child) {
        super(messageBuilder(child));
        this.child = child;
    }

    public Node parent() {
        return child;
    }

    private static String messageBuilder(Node parent) {
        if (parent == null)
            return "Child was null";
        return "Child was " + parent.getClass().getSimpleName();
    }

}
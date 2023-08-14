package dev.redio.chorus.parser.exception;

import dev.redio.chorus.parser.node.Node;

public class IllegalChildException extends IllegalNodeException {
     private final Node child;
     private final Node parent;

    public IllegalChildException(Node child, Node parent) {
        super(messageBuilder(child, parent));
        this.child = child;
        this.parent = parent;
    }

    public Node child() {
        return child;
    }

    public Node parent() {
        return parent;
    }

    private static String messageBuilder(Node child, Node parent) {
        if (child == null)
            return "Child in" + parent.getClass().getName() + " was null";
        return "Child " + child.getClass().getSimpleName() + " is not allowed in Parent" + parent.getClass().getSimpleName();
    }

}
package dev.redio.chorus.parser.exception;

import dev.redio.chorus.parser.node.Modifier;
import dev.redio.chorus.parser.node.Node;

public class IllegalModifierException extends RuntimeException {
    
    private final Modifier modifier;
    private final Node node;

    public IllegalModifierException(Modifier modifier, Node node) {
        super(messageBuilder(modifier, node));
        this.modifier = modifier;
        this.node = node;
    }

    public Modifier modifier() {
        return modifier;
    }

    public Node node() {
        return node;
    }

    private static String messageBuilder(Modifier modifier, Node node) {
        if (modifier == null)
            return "Modifier was null";
        return "Modifier " + modifier.getClass().getSimpleName() + "is not allowed for Node " + node.getClass().getSimpleName();
    }
}

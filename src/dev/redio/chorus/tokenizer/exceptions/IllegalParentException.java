package dev.redio.chorus.tokenizer.exceptions;

import dev.redio.chorus.tokenizer.Token;

public class IllegalParentException extends IllegalTokenException {
    
    private Token parent;

    public IllegalParentException(Token parent) {
        super(messageBuilder(parent));
        this.parent = parent;
    }

    public Token parent() {
        return parent;
    }

    private static String messageBuilder(Token parent) {
        if (parent == null)
            return "Parent was null";
        return "Parent was " + parent.getClass().getSimpleName();
    }
}

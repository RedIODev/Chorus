package dev.redio.chorus.tokenizer.exceptions;

import dev.redio.chorus.tokenizer.Token;

public class IllegalChildException extends IllegalTokenException {
     private Token child;

    public IllegalChildException(Token child) {
        super(messageBuilder(child));
        this.child = child;
    }

    public Token parent() {
        return child;
    }

    private static String messageBuilder(Token parent) {
        if (parent == null)
            return "Child was null";
        return "Child was " + parent.getClass().getSimpleName();
    }

}
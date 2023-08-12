package dev.redio.chorus.tokenizer;

import java.util.Optional;

import dev.redio.chorus.tokenizer.exceptions.IllegalChildException;
import dev.redio.chorus.tokenizer.exceptions.IllegalParentException;

public class NamespaceToken implements Token {

    private final Token parent;
    private final Token[] childs;

    public NamespaceToken(Token parent, Token... childs) {
        this.parent = switch (parent) {
            case NamespaceToken nt -> nt;
            case FileToken ft -> ft;
            case null -> throw new IllegalParentException(parent);
            default -> throw new  IllegalParentException(parent);
        };
        if (childs == null) {
            this.childs = new Token[0];
            return;
        }
        
        for (Token child : childs) {
            switch (child) {
                case NamespaceToken nt -> {}

                case null -> throw new IllegalChildException(child);
                default -> throw new IllegalChildException(child);
            }
        }
        this.childs = childs;

    }

    @Override
    public Optional<Token> parent() {
        return Optional.of(parent);
    }

    @Override
    public Token[] childs() {
       return childs;
    }

    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder("namespace {");
        for (var child : childs) {
            builder.append(FormatHelper.indent(child.toString()));
            builder.append(System.lineSeparator());
        }

        builder.append("}");
        return builder.toString();
    }
    
}

package dev.redio.chorus.tokenizer;

import java.util.Optional;

import dev.redio.chorus.tokenizer.exceptions.IllegalParentException;

public class ImportToken implements Token {

    private final Token parent;

    public ImportToken(Token parent) {
        this.parent = switch (parent) {
            case NamespaceToken nt -> nt;
            case null -> throw new IllegalParentException(parent);
            default -> throw new IllegalParentException(parent);
        };
    }
    @Override
    public Optional<Token> parent() {
        return Optional.of(parent);
    }

    @Override
    public Token[] childs() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'childs'");
    }


  
    
}

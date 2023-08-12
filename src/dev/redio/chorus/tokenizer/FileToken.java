package dev.redio.chorus.tokenizer;

import java.util.Optional;

public class FileToken implements Token {

    

    @Override
    public Optional<Token> parent() {
       return Optional.empty();
    }

    @Override
    public Token[] childs() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'childs'");
    }

    
}

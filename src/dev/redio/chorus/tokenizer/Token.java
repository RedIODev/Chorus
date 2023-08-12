package dev.redio.chorus.tokenizer;

import java.util.Optional;

public interface Token {
    Optional<Token> parent();
    Token[] childs();
}

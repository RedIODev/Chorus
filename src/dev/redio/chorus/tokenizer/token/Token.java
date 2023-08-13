package dev.redio.chorus.tokenizer.token;

public interface Token {
    String raw();
    int line();
    int column();
}

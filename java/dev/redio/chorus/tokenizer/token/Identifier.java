package dev.redio.chorus.tokenizer.token;

public record Identifier(String raw, int line, int column) implements Token {
    @Override
    public String toString() {
        return "Identifier(" + raw + ")@:" + (line+1) + "C:" + (column+1);
    }
}
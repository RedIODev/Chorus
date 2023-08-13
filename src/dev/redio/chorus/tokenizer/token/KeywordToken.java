package dev.redio.chorus.tokenizer.token;

public record KeywordToken(Keyword keyword, int line, int column) implements Token {

    @Override
    public String raw() {
        return keyword.raw();
    }

    @Override
    public String toString() {
        return keyword.toString() + "@Ln:" + (line+1) + "Col:" + (column+1);
    }
   
    
}

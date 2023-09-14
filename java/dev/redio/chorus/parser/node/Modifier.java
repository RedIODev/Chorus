package dev.redio.chorus.parser.node;

public enum Modifier {
    STATIC("static"),
    UNSAFE("unsafe"),
    CONST("const"),
    UNSIGNED("unsigned"),
    EXTERN("extern"),
    INLINE("inline"),
    ASYNC("async"),
    ;

    private final String raw;
    private Modifier(String raw) {
        this.raw = raw;
    }

    public String raw() {
        return raw;
    }

    @Override
    public String toString() {
        return "Modifier[" + raw + "]";
    }
}

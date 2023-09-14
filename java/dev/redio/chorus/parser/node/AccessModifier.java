package dev.redio.chorus.parser.node;

public enum AccessModifier {
    PUBLIC("public"),
    LOCAL("local"),
    DEFAULT("");

    private final String raw;
    private AccessModifier(String raw) {
        this.raw = raw;
    }

    public String raw() {
        return raw;
    }

    @Override
    public String toString() {
        return "AccessModifier[" + raw + "]";
    }
}
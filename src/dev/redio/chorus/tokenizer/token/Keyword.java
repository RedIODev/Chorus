package dev.redio.chorus.tokenizer.token;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;

public enum Keyword {
    CURLY_BRACE_OPEN("{"),
    CURLY_BRACE_CLOSED("}"),
    SQUARE_BRACE_OPEN("["),
    SQUARE_BRACE_CLOSED("]"),
    ROUND_BRACE_OPEN("("),
    ROUND_BRACE_CLOSED(")"),
    DIAMOND_BRACE_OPEN("<"),
    DIAMOND_BRACE_CLOSED(">"),
    DOUBLE_QUOTE("\""),
    SINGLE_QUOTE("'"),
    EXCLAMATION_MARK("!"),
    QUESTION_MARK("?"),
    PIPE("|"),
    SLASH("/"),
    BACKSLASH("\\"),
    PERCENT("%"),
    AND("&"),
    EQUALS("="),
    STAR("*"),
    PLUS("+"),
    MINUS("-"),
    UNDERSCORE("_"),
    COMMA(","),
    SEMI_COLON(";"),
    COLON(":"),
    PERIOD("."),
    TILDE("~"),
    HASHTAG("#"),
    CARET("^"),
    DEGREE("°"),
    AT("@"),

    COMMENT_LINE("//"),
    COMMENT_START("/*"),
    COMMENT_END("*/"),
    DOUBLE_EQUALS("=="),
    SHIFT_LEFT("<<"),
    SHIFT_RIGHT(">>"),
    DOUBLE_AND("&&"),
    DOUBLE_PIPE("||"),
    LESS_EQUALS("<="),
    MORE_EQUALS(">="),
    NOT_EQUALS("!="),
    ARROW("->"),
    FAT_ARROW("=>"),
    STAR_EQUALS("*="),
    SLASH_EQUALS("/="),
    PLUS_EQUALS("+="),
    MINUS_EQUALS("-="),
    PERCENT_EQUALS("%="),
    AND_EQUALS("&="),
    PIPE_EQUALS("|="),
    TILDE_EQUALS("~="),
    CARET_EQUALS("^="),
    QUESTION_MARK_EQUALS("?="),
    HASH_EQUALS("#="),
    DEGREE_EQUALS("°="),
    AT_EQUALS("@="),
    DOUBLE_DOUBLE_COLON("::"),

    SHIFT_LEFT_EQUALS("<<="),
    SHIFT_RIGHT_EQUALS(">>="),
    DOUBLE_AND_EQUALS("&&="),
    DOUBLE_PIPE_EQUALS("||="),


    NAMESPACE("namespace"),
    IMPORT("import"),
    PUBLIC("public"),
    LOCAL("local"),
    CONST("const"),
    MUT("mut"),
    STRUCT("struct"),
    VAR("var"),
    UNSAFE("unsafe"),
    EXTERN("extern"),
    INLINE("inline"),
    INTERFACE("interface"),
    //FP("fp"),
    TYPE("type"),
    AS("as"),
    FN("fn"),
    //HEAP("heap"),     //contextual
    //STACK("stack"),   //contextual
    BREAK("break"),
    CONTINUE("continue"),
    IF("if"),
    ELSE("else"),
    SWITCH("switch"),
    FOR("for"),
    WHILE("while"),
    RETURN("return"),
    ENUM("enum"),
    UNION("union"),
    STATIC("static"),
    STAGED("staged"),
    YIELD("yield"),
    IN("in"),
    UNSIGNED("unsigned"),
    WITH("with"),
    WHERE("where"),
    SUPER("super"),
    SATISFIES("satisfies"),
    ALLOC("alloc"),
    FUNCS("funcs"),
    REF("ref"),
    SELF("Self"),
    
    __LINE("__line"),
    __FILE("__file"),

    ;

    private static final Comparator<Keyword> COMPARATOR = (t1, t2) -> t1.raw().length() - t2.raw().length();
    
    public static final List<Keyword> SORTED_KEYWORDS;
    public static final List<Keyword> SYMBOLIC_KEYWORDS;
    
    static {
        var keywords = Keyword.values();
        var list = new ArrayList<Keyword>();
        for (var token : keywords) 
            list.add(token);
        list.sort(COMPARATOR.reversed());
        SORTED_KEYWORDS = Collections.unmodifiableList(list);
        var symbolicList = list.stream()
                .filter(Keyword::isSymbolic)
                .toList();
        SYMBOLIC_KEYWORDS = Collections.unmodifiableList(symbolicList);
    }

    private final String raw;

    private Keyword(String raw) {
        this.raw = raw;
    }

    public String raw() {
        return this.raw;
    }

    public boolean isSymbolic() {
        for (int i = 0; i < raw.length(); i++) {
            if (Character.isAlphabetic(raw.charAt(i)))
                return false;
        }
        return true;
    }

    @Override
    public String toString() {
        return this.name() + "(" + this.raw + ")";
    }
}

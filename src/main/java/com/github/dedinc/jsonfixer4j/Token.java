package com.github.dedinc.jsonfixer4j;

public class Token {
    public final TokenKind kind;
    public final String value;

    public static final Token TRUE = new Token(TokenKind.TRUE, "true");
    public static final Token FALSE = new Token(TokenKind.FALSE, "false");
    public static final Token NULL = new Token(TokenKind.NULL, "null");

    public Token(TokenKind kind, String value) {
        this.kind = kind;
        this.value = value;
    }

    @Override
    public String toString() {
        return "Token(" + kind + ", " + value + ")";
    }
}

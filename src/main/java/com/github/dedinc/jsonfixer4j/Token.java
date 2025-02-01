package com.github.dedinc.jsonfixer4j;

class Token {
    TokenKind kind;
    String value;

    public Token(TokenKind kind, String value) {
        this.kind = kind;
        this.value = value;
    }

    @Override
    public String toString() {
        return "Token(" + kind + ", " + value + ")";
    }
}
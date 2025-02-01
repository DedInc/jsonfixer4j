package com.github.dedinc.jsonfixer4j;

public enum TokenKind {
    LBRACE,      // {
    RBRACE,      // }
    LBRACKET,    // [
    RBRACKET,    // ]
    COLON,       // :
    COMMA,       // ,
    STRING,      // "..."
    NUMBER,      // numeric literal
    TRUE,        // true
    FALSE,       // false
    NULL,        // null
    EOF,         // end of file/input
    UNKNOWN      // unrecognized token
}
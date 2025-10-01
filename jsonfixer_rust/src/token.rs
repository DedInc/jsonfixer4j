/// Token types for JSON parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Colon,     // :
    Comma,     // ,
    String,    // "..."
    Number,    // 123, 45.67
    True,      // true
    False,     // false
    Null,      // null
    Eof,       // End of input
    Unknown,   // Unknown token
}

/// Represents a single token with its type and optional value
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, value: Option<String>) -> Self {
        Self { kind, value }
    }

    #[inline]
    pub fn new_simple(kind: TokenKind) -> Self {
        Self { kind, value: None }
    }

    #[inline]
    pub fn true_token() -> Self {
        Self::new(TokenKind::True, Some("true".to_string()))
    }

    #[inline]
    pub fn false_token() -> Self {
        Self::new(TokenKind::False, Some("false".to_string()))
    }

    #[inline]
    pub fn null_token() -> Self {
        Self::new(TokenKind::Null, Some("null".to_string()))
    }
}


use crate::token::{Token, TokenKind};
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

static NUM_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^-?\d+(\.\d+)?([eE][+-]?\d+)?$").unwrap()
});

static ID_PATTERN: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"^[A-Za-z_]\w*$")
        .case_insensitive(false)
        .build()
        .unwrap()
});

/// High-performance JSON tokenizer optimized for large inputs
pub struct JSONTokenizer {
    // Reusable buffer for string building
    string_buffer: String,
}

impl JSONTokenizer {
    pub fn new() -> Self {
        Self {
            string_buffer: String::with_capacity(256),
        }
    }

    /// Tokenize JSON input with optimized performance for large strings
    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let bytes = input.as_bytes();
        let length = bytes.len();
        let mut tokens = Vec::with_capacity(length / 4); // Estimate: avg 4 bytes per token
        let mut i = 0;

        while i < length {
            let c = bytes[i] as char;

            // Skip whitespace
            if c.is_ascii_whitespace() {
                i += 1;
                continue;
            }

            match c {
                '{' => {
                    tokens.push(Token::new(TokenKind::LBrace, Some("{".to_string())));
                    i += 1;
                }
                '}' => {
                    tokens.push(Token::new(TokenKind::RBrace, Some("}".to_string())));
                    i += 1;
                }
                '[' => {
                    tokens.push(Token::new(TokenKind::LBracket, Some("[".to_string())));
                    i += 1;
                }
                ']' => {
                    tokens.push(Token::new(TokenKind::RBracket, Some("]".to_string())));
                    i += 1;
                }
                ':' => {
                    tokens.push(Token::new(TokenKind::Colon, Some(":".to_string())));
                    i += 1;
                }
                ',' => {
                    tokens.push(Token::new(TokenKind::Comma, Some(",".to_string())));
                    i += 1;
                }
                '"' => {
                    i += 1;
                    let (string_value, new_pos) = self.parse_string(input, i, length);
                    tokens.push(Token::new(TokenKind::String, Some(string_value)));
                    i = new_pos;
                }
                _ => {
                    if c.is_ascii_alphabetic() || c == '-' || c.is_ascii_digit() {
                        let start = i;
                        while i < length {
                            let ch = bytes[i] as char;
                            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '+' || ch == '.' || ch == 'e' || ch == 'E' {
                                i += 1;
                            } else {
                                break;
                            }
                        }
                        let raw_val = &input[start..i];
                        tokens.push(self.correct_literal(raw_val));
                    } else {
                        i += 1;
                    }
                }
            }
        }

        tokens.push(Token::new_simple(TokenKind::Eof));
        tokens
    }

    /// Optimized string parsing with escape sequence handling
    #[inline]
    fn parse_string(&mut self, input: &str, start: usize, length: usize) -> (String, usize) {
        self.string_buffer.clear();
        let bytes = input.as_bytes();
        let mut i = start;

        while i < length {
            let c = bytes[i] as char;
            
            if c == '"' {
                i += 1;
                break;
            } else if c == '\\' {
                if i + 1 < length {
                    self.string_buffer.push(c);
                    self.string_buffer.push(bytes[i + 1] as char);
                    i += 2;
                } else {
                    i += 1;
                    break;
                }
            } else {
                self.string_buffer.push(c);
                i += 1;
            }
        }

        (self.string_buffer.clone(), i)
    }

    /// Correct and identify literal tokens (true, false, null, numbers, identifiers)
    #[inline]
    fn correct_literal(&self, raw: &str) -> Token {
        let len = raw.len();

        // Fast path for boolean and null literals
        if len <= 5 {
            let first = raw.as_bytes()[0] as char;
            match first.to_ascii_lowercase() {
                't' => {
                    if self.match_literal(raw, "true") {
                        return Token::true_token();
                    }
                }
                'f' => {
                    if self.match_literal(raw, "false") {
                        return Token::false_token();
                    }
                }
                'n' => {
                    if self.match_literal(raw, "null") {
                        return Token::null_token();
                    }
                }
                _ => {}
            }
        }

        // Check if it's a number
        if NUM_PATTERN.is_match(raw) {
            return Token::new(TokenKind::Number, Some(raw.to_string()));
        }

        // Check if it's a valid identifier (treat as string)
        if ID_PATTERN.is_match(raw) {
            return Token::new(TokenKind::String, Some(raw.to_string()));
        }

        Token::new(TokenKind::Unknown, Some(raw.to_string()))
    }

    /// Match partial literal against full literal (case-insensitive)
    #[inline]
    fn match_literal(&self, raw: &str, full: &str) -> bool {
        let raw_bytes = raw.as_bytes();
        let full_bytes = full.as_bytes();
        let len = raw_bytes.len();

        if len > full_bytes.len() {
            return false;
        }

        for i in 0..len {
            if (raw_bytes[i] as char).to_ascii_lowercase() != (full_bytes[i] as char) {
                return false;
            }
        }

        true
    }
}

impl Default for JSONTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_basic() {
        let mut tokenizer = JSONTokenizer::new();
        let tokens = tokenizer.tokenize("{\"key\":42}");

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::LBrace);
        assert_eq!(tokens[1].kind, TokenKind::String);
        assert_eq!(tokens[1].value.as_ref().unwrap(), "key");
        assert_eq!(tokens[2].kind, TokenKind::Colon);
        assert_eq!(tokens[3].kind, TokenKind::Number);
        assert_eq!(tokens[3].value.as_ref().unwrap(), "42");
        assert_eq!(tokens[4].kind, TokenKind::RBrace);
        assert_eq!(tokens[5].kind, TokenKind::Eof);
    }

    #[test]
    fn test_tokenizer_incomplete_string() {
        let mut tokenizer = JSONTokenizer::new();
        let tokens = tokenizer.tokenize("{\"key\":\"incomplete");

        // tokens: { "key" : "incomplete" EOF
        // Index:  0   1   2      3       4
        assert_eq!(tokens[3].kind, TokenKind::String);
        assert_eq!(tokens[3].value.as_ref().unwrap(), "incomplete");
    }

    #[test]
    fn test_tokenizer_partial_literals() {
        let mut tokenizer = JSONTokenizer::new();
        
        let tokens = tokenizer.tokenize("tr");
        assert_eq!(tokens[0].kind, TokenKind::True);
        
        let tokens = tokenizer.tokenize("fals");
        assert_eq!(tokens[0].kind, TokenKind::False);
        
        let tokens = tokenizer.tokenize("nul");
        assert_eq!(tokens[0].kind, TokenKind::Null);
    }

    #[test]
    fn test_tokenizer_scientific_notation() {
        let mut tokenizer = JSONTokenizer::new();
        let tokens = tokenizer.tokenize("1.5e10");
        
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[0].value.as_ref().unwrap(), "1.5e10");
    }
}


use crate::token::{Token, TokenKind};

/// Fixes mismatched brackets and braces in token stream
pub struct JSONTokenFixer;

impl JSONTokenFixer {
    /// Fix tokens by ensuring all opening brackets/braces have matching closing ones
    /// Optimized to reduce allocations and cloning
    pub fn fix_tokens(tokens: Vec<Token>) -> Vec<Token> {
        let mut fixed = Vec::with_capacity(tokens.len() + 16); // Extra space for potential fixes
        let mut stack: Vec<TokenKind> = Vec::with_capacity(32); // Stack for tracking open brackets
        let mut eof_token: Option<Token> = None;

        for token in tokens {
            match token.kind {
                TokenKind::Eof => {
                    // Save EOF token to add at the very end
                    eof_token = Some(token);
                }
                TokenKind::LBrace | TokenKind::LBracket => {
                    let expected_close = if token.kind == TokenKind::LBrace {
                        TokenKind::RBrace
                    } else {
                        TokenKind::RBracket
                    };
                    fixed.push(token);
                    stack.push(expected_close);
                }
                TokenKind::RBrace | TokenKind::RBracket => {
                    if let Some(&expected) = stack.last() {
                        if token.kind == expected {
                            // Matching closing bracket
                            stack.pop();
                            fixed.push(token);
                        } else {
                            // Mismatched closing bracket - insert correct one first
                            fixed.push(Self::create_closing_token(expected));
                            stack.pop();

                            // Check if current token matches new top of stack
                            if let Some(&new_expected) = stack.last() {
                                if token.kind == new_expected {
                                    stack.pop();
                                    fixed.push(token);
                                }
                            }
                        }
                    } else {
                        // Closing bracket without opening - skip it
                        // (or we could add it anyway, depending on desired behavior)
                    }
                }
                _ => {
                    fixed.push(token);
                }
            }
        }

        // Close any remaining open brackets
        while let Some(expected) = stack.pop() {
            fixed.push(Self::create_closing_token(expected));
        }

        // Add EOF token at the very end if it exists
        if let Some(eof) = eof_token {
            fixed.push(eof);
        }

        fixed
    }

    /// Create a closing bracket/brace token
    #[inline]
    fn create_closing_token(kind: TokenKind) -> Token {
        match kind {
            TokenKind::RBrace => Token::new(TokenKind::RBrace, Some("}".to_string())),
            TokenKind::RBracket => Token::new(TokenKind::RBracket, Some("]".to_string())),
            _ => Token::new_simple(TokenKind::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_missing_closing_brace() {
        let tokens = vec![
            Token::new(TokenKind::LBrace, Some("{".to_string())),
            Token::new(TokenKind::String, Some("key".to_string())),
            Token::new(TokenKind::Colon, Some(":".to_string())),
            Token::new(TokenKind::Number, Some("42".to_string())),
            Token::new_simple(TokenKind::Eof),
        ];

        let fixed = JSONTokenFixer::fix_tokens(tokens);

        // Should have added closing brace - check that RBrace exists in the fixed tokens
        let has_rbrace = fixed.iter().any(|t| t.kind == TokenKind::RBrace);
        assert!(has_rbrace, "Fixed tokens should contain RBrace");

        // The closing brace should be added before EOF
        let rbrace_pos = fixed.iter().position(|t| t.kind == TokenKind::RBrace).unwrap();
        let eof_pos = fixed.iter().position(|t| t.kind == TokenKind::Eof).unwrap();
        assert!(rbrace_pos < eof_pos, "RBrace should come before EOF");
    }

    #[test]
    fn test_fix_mismatched_brackets() {
        let tokens = vec![
            Token::new(TokenKind::LBrace, Some("{".to_string())),
            Token::new(TokenKind::String, Some("arr".to_string())),
            Token::new(TokenKind::Colon, Some(":".to_string())),
            Token::new(TokenKind::LBracket, Some("[".to_string())),
            Token::new(TokenKind::Number, Some("1".to_string())),
            Token::new(TokenKind::RBrace, Some("}".to_string())), // Wrong: should be ]
            Token::new_simple(TokenKind::Eof),
        ];

        let fixed = JSONTokenFixer::fix_tokens(tokens);
        
        // Should fix the mismatched bracket
        assert!(fixed.iter().any(|t| t.kind == TokenKind::RBracket));
    }

    #[test]
    fn test_nested_structures() {
        let tokens = vec![
            Token::new(TokenKind::LBrace, Some("{".to_string())),
            Token::new(TokenKind::String, Some("a".to_string())),
            Token::new(TokenKind::Colon, Some(":".to_string())),
            Token::new(TokenKind::LBracket, Some("[".to_string())),
            Token::new(TokenKind::LBrace, Some("{".to_string())),
            // Missing all closing brackets
            Token::new_simple(TokenKind::Eof),
        ];

        let fixed = JSONTokenFixer::fix_tokens(tokens);
        
        // Should add }, ], }
        let closing_count = fixed.iter()
            .filter(|t| t.kind == TokenKind::RBrace || t.kind == TokenKind::RBracket)
            .count();
        assert_eq!(closing_count, 3);
    }
}


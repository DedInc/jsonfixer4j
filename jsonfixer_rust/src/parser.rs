use crate::token::{Token, TokenKind};
use serde_json::{Map, Value};

/// Result of parsing operation
#[derive(Debug)]
pub struct ParseResult {
    pub value: Option<Value>,
    pub index: usize,
}

/// High-performance JSON parser optimized for memory efficiency
pub struct JSONParser;

impl JSONParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse tokens into JSON value
    pub fn parse(&self, token_list: &[Token], idx: usize) -> ParseResult {
        if idx >= token_list.len() {
            return ParseResult {
                value: None,
                index: idx,
            };
        }

        let token = &token_list[idx];
        match token.kind {
            TokenKind::LBrace => self.parse_object(token_list, idx + 1),
            TokenKind::LBracket => self.parse_array(token_list, idx + 1),
            TokenKind::String => ParseResult {
                value: Some(Value::String(
                    token.value.as_ref().map(|s| s.clone()).unwrap_or_default(),
                )),
                index: idx + 1,
            },
            TokenKind::Number => self.parse_number(token, idx),
            TokenKind::True => ParseResult {
                value: Some(Value::Bool(true)),
                index: idx + 1,
            },
            TokenKind::False => ParseResult {
                value: Some(Value::Bool(false)),
                index: idx + 1,
            },
            TokenKind::Null => ParseResult {
                value: Some(Value::Null),
                index: idx + 1,
            },
            TokenKind::RBrace | TokenKind::RBracket | TokenKind::Eof => ParseResult {
                value: None,
                index: idx,
            },
            _ => ParseResult {
                value: None,
                index: idx + 1,
            },
        }
    }

    /// Parse number token with optimized conversion
    #[inline]
    fn parse_number(&self, token: &Token, idx: usize) -> ParseResult {
        let value_str = token.value.as_ref().map(|s| s.as_str()).unwrap_or("0");

        if value_str.contains('.') || value_str.contains('e') || value_str.contains('E') {
            // Floating point number
            if let Ok(num) = value_str.parse::<f64>() {
                return ParseResult {
                    value: Some(Value::Number(
                        serde_json::Number::from_f64(num)
                            .unwrap_or_else(|| serde_json::Number::from(0)),
                    )),
                    index: idx + 1,
                };
            }
        } else {
            // Integer number
            if let Ok(num) = value_str.parse::<i64>() {
                return ParseResult {
                    value: Some(Value::Number(serde_json::Number::from(num))),
                    index: idx + 1,
                };
            }
        }

        // Fallback to string if parsing fails
        ParseResult {
            value: Some(Value::String(value_str.to_string())),
            index: idx + 1,
        }
    }

    /// Parse JSON object with optimized memory allocation
    fn parse_object(&self, token_list: &[Token], start_idx: usize) -> ParseResult {
        let mut result = Map::with_capacity(16); // Pre-allocate for typical object size
        let mut expect_comma = false;
        let size = token_list.len();
        let mut idx = start_idx;

        while idx < size {
            let token = &token_list[idx];

            // Check for object end
            if token.kind == TokenKind::RBrace || token.kind == TokenKind::Eof {
                return ParseResult {
                    value: Some(Value::Object(result)),
                    index: idx + 1,
                };
            }

            // Handle comma expectation
            if expect_comma {
                if token.kind == TokenKind::Comma {
                    idx += 1;
                    expect_comma = false;
                    continue;
                } else if token.kind == TokenKind::RBrace {
                    return ParseResult {
                        value: Some(Value::Object(result)),
                        index: idx + 1,
                    };
                }
                // Missing comma - continue anyway (auto-fix)
                expect_comma = false;
            }

            // Parse key-value pair
            if token.kind == TokenKind::String {
                let key = token.value.as_ref().map(|s| s.clone()).unwrap_or_default();
                idx += 1;

                // Expect colon
                if idx < size && token_list[idx].kind == TokenKind::Colon {
                    idx += 1;
                    
                    // Parse value
                    let pr = self.parse(token_list, idx);
                    if let Some(value) = pr.value {
                        result.insert(key, value);
                    }
                    idx = pr.index;
                    expect_comma = true;
                } else {
                    // Missing colon - treat key as standalone value with null
                    result.insert(key, Value::Null);
                    expect_comma = true;
                }
            } else {
                // Unexpected token - skip it
                idx += 1;
            }
        }

        ParseResult {
            value: Some(Value::Object(result)),
            index: idx,
        }
    }

    /// Parse JSON array with optimized memory allocation
    fn parse_array(&self, token_list: &[Token], start_idx: usize) -> ParseResult {
        let mut result = Vec::with_capacity(16); // Pre-allocate for typical array size
        let mut expect_comma = false;
        let size = token_list.len();
        let mut idx = start_idx;

        while idx < size {
            let token = &token_list[idx];

            // Check for array end
            if token.kind == TokenKind::RBracket || token.kind == TokenKind::Eof {
                return ParseResult {
                    value: Some(Value::Array(result)),
                    index: idx + 1,
                };
            }

            // Handle comma expectation
            if expect_comma {
                if token.kind == TokenKind::Comma {
                    idx += 1;
                    expect_comma = false;
                    continue;
                } else if token.kind == TokenKind::RBracket {
                    return ParseResult {
                        value: Some(Value::Array(result)),
                        index: idx + 1,
                    };
                }
                // Missing comma - continue anyway (auto-fix)
            }
            expect_comma = false;

            // Check if token is a valid value
            let valid_token = matches!(
                token.kind,
                TokenKind::LBrace
                    | TokenKind::LBracket
                    | TokenKind::String
                    | TokenKind::Number
                    | TokenKind::True
                    | TokenKind::False
                    | TokenKind::Null
            );

            if !valid_token {
                return ParseResult {
                    value: Some(Value::Array(result)),
                    index: idx,
                };
            }

            // Parse array element
            let pr = self.parse(token_list, idx);
            if let Some(value) = pr.value {
                result.push(value);
            }
            idx = pr.index;
            expect_comma = true;
        }

        ParseResult {
            value: Some(Value::Array(result)),
            index: idx,
        }
    }
}

impl Default for JSONParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_object() {
        let parser = JSONParser::new();
        let tokens = vec![
            Token::new(TokenKind::LBrace, Some("{".to_string())),
            Token::new(TokenKind::String, Some("key".to_string())),
            Token::new(TokenKind::Colon, Some(":".to_string())),
            Token::new(TokenKind::Number, Some("42".to_string())),
            Token::new(TokenKind::RBrace, Some("}".to_string())),
            Token::new_simple(TokenKind::Eof),
        ];

        let result = parser.parse(&tokens, 0);
        assert!(result.value.is_some());
        
        if let Some(Value::Object(obj)) = result.value {
            assert_eq!(obj.get("key").unwrap(), &Value::Number(serde_json::Number::from(42)));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_parse_array() {
        let parser = JSONParser::new();
        let tokens = vec![
            Token::new(TokenKind::LBracket, Some("[".to_string())),
            Token::new(TokenKind::Number, Some("1".to_string())),
            Token::new(TokenKind::Comma, Some(",".to_string())),
            Token::new(TokenKind::Number, Some("2".to_string())),
            Token::new(TokenKind::RBracket, Some("]".to_string())),
            Token::new_simple(TokenKind::Eof),
        ];

        let result = parser.parse(&tokens, 0);
        
        if let Some(Value::Array(arr)) = result.value {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array");
        }
    }
}


use std::collections::VecDeque;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};
use serde_json::{Map, Value};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    LBrace,    
    RBrace,    
    LBracket,  
    RBracket,  
    Colon,     
    Comma,     
    String,    
    Number,    
    True,      
    False,     
    Null,      
    Eof,       
    Unknown,   
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<String>) -> Self {
        Self { kind, value }
    }

    pub fn true_token() -> Self {
        Self::new(TokenKind::True, Some("true".to_string()))
    }

    pub fn false_token() -> Self {
        Self::new(TokenKind::False, Some("false".to_string()))
    }

    pub fn null_token() -> Self {
        Self::new(TokenKind::Null, Some("null".to_string()))
    }
}

static NUM_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^-?\d+(\.\d+)?$").unwrap()
});

static ID_PATTERN: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"^[A-Za-z_]\w*$")
        .case_insensitive(false)
        .build()
        .unwrap()
});

pub struct JSONTokenizer;

impl JSONTokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(input.len() / 2);
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        let length = chars.len();

        while i < length {
            let c = chars[i];

            if c.is_whitespace() {
                i += 1;
                continue;
            }

            match c {
                '{' => {
                    tokens.push(Token::new(TokenKind::LBrace, Some("{".to_string())));
                    i += 1;
                },
                '}' => {
                    tokens.push(Token::new(TokenKind::RBrace, Some("}".to_string())));
                    i += 1;
                },
                '[' => {
                    tokens.push(Token::new(TokenKind::LBracket, Some("[".to_string())));
                    i += 1;
                },
                ']' => {
                    tokens.push(Token::new(TokenKind::RBracket, Some("]".to_string())));
                    i += 1;
                },
                ':' => {
                    tokens.push(Token::new(TokenKind::Colon, Some(":".to_string())));
                    i += 1;
                },
                ',' => {
                    tokens.push(Token::new(TokenKind::Comma, Some(",".to_string())));
                    i += 1;
                },
                '"' => {
                    i += 1;
                    let mut sb = String::new();
                    while i < length {
                        let current = chars[i];
                        if current == '"' {
                            i += 1;
                            break;
                        } else if current == '\\' {
                            if i + 1 < length {
                                sb.push(current);
                                sb.push(chars[i + 1]);
                                i += 2;
                            } else {
                                i += 1;
                                break;
                            }
                        } else {
                            sb.push(current);
                            i += 1;
                        }
                    }
                    tokens.push(Token::new(TokenKind::String, Some(sb)));
                },
                _ => {
                    if c.is_alphabetic() || c == '-' || c.is_numeric() {
                        let start = i;
                        while i < length {
                            let ch = chars[i];
                            if ch.is_alphanumeric() || ch == '-' || ch == '+' || ch == '.' {
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

        tokens.push(Token::new(TokenKind::Eof, None));
        tokens
    }

    fn correct_literal(&self, raw: &str) -> Token {
        let len = raw.len();

        if len <= 5 {
            let first = raw.chars().next().unwrap_or(' ').to_ascii_lowercase();
            match first {
                't' => {
                    if self.match_literal(raw, "true") {
                        return Token::true_token();
                    }
                },
                'f' => {
                    if self.match_literal(raw, "false") {
                        return Token::false_token();
                    }
                },
                'n' => {
                    if self.match_literal(raw, "null") {
                        return Token::null_token();
                    }
                },
                _ => {}
            }
        }

        if NUM_PATTERN.is_match(raw) {
            return Token::new(TokenKind::Number, Some(raw.to_string()));
        }

        if ID_PATTERN.is_match(raw) {
            return Token::new(TokenKind::String, Some(raw.to_string()));
        }

        Token::new(TokenKind::Unknown, Some(raw.to_string()))
    }

    fn match_literal(&self, raw: &str, full: &str) -> bool {
        let raw_chars: Vec<char> = raw.chars().collect();
        let full_chars: Vec<char> = full.chars().collect();
        let len = raw_chars.len();

        if len > full_chars.len() {
            return false;
        }

        for i in 0..len {
            if raw_chars[i].to_ascii_lowercase() != full_chars[i] {
                return false;
            }
        }

        true
    }
}

pub struct JSONTokenFixer;

impl JSONTokenFixer {
    pub fn fix_tokens(tokens: Vec<Token>) -> Vec<Token> {
        let mut fixed = Vec::with_capacity(tokens.len());
        let mut stack = VecDeque::with_capacity(tokens.len());

        for token in tokens {
            match token.kind {
                TokenKind::LBrace | TokenKind::LBracket => {
                    fixed.push(token.clone());
                    stack.push_back(if token.kind == TokenKind::LBrace { 
                        TokenKind::RBrace 
                    } else { 
                        TokenKind::RBracket 
                    });
                },
                TokenKind::RBrace | TokenKind::RBracket => {
                    if !stack.is_empty() && token.kind == stack[stack.len() - 1] {
                        stack.pop_back();
                        fixed.push(token);
                    } else {
                        let expected_token = if stack.is_empty() { None } else { Some(stack[stack.len() - 1]) };
                        fixed.push(Self::create_matching_token(expected_token, Some(&token)));
                        if !stack.is_empty() && token.kind == stack[stack.len() - 1] {
                            stack.pop_back();
                            fixed.push(token);
                        }
                    }
                },
                _ => {
                    fixed.push(token);
                },
            }
        }

        while let Some(expected_token) = stack.pop_back() {
            fixed.push(Self::create_matching_token(Some(expected_token), None));
        }

        fixed
    }

    fn create_matching_token(expected_token: Option<TokenKind>, original_token: Option<&Token>) -> Token {
        if let Some(expected) = expected_token {
            match expected {
                TokenKind::RBrace => Token::new(TokenKind::RBrace, Some("}".to_string())),
                TokenKind::RBracket => Token::new(TokenKind::RBracket, Some("]".to_string())),
                _ => original_token.cloned().unwrap_or_else(|| Token::new(TokenKind::Unknown, None)),
            }
        } else {
            original_token.cloned().unwrap_or_else(|| Token::new(TokenKind::Unknown, None))
        }
    }
}

pub struct JSONParser;

impl JSONParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, token_list: &[Token], idx: usize) -> ParseResult {
        if idx >= token_list.len() {
            return ParseResult { value: None, index: idx };
        }

        let token = &token_list[idx];
        match token.kind {
            TokenKind::LBrace => self.parse_object(token_list, idx + 1),
            TokenKind::LBracket => self.parse_array(token_list, idx + 1),
            TokenKind::String => ParseResult {
                value: Some(Value::String(token.value.clone().unwrap_or_default())),
                index: idx + 1,
            },
            TokenKind::Number => {
                let value_str = token.value.clone().unwrap_or_default();
                if value_str.contains('.') {

                    if let Ok(num) = value_str.parse::<f64>() {
                        return ParseResult {
                            value: Some(Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))),
                            index: idx + 1,
                        };
                    }
                } else {

                    if let Ok(num) = value_str.parse::<i64>() {
                        return ParseResult {
                            value: Some(Value::Number(serde_json::Number::from(num))),
                            index: idx + 1,
                        };
                    }
                }

                ParseResult {
                    value: Some(Value::String(value_str)),
                    index: idx + 1,
                }
            },
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

    fn parse_object(&self, token_list: &[Token], idx: usize) -> ParseResult {
        let mut result = Map::new();
        let mut expect_comma = false;
        let size = token_list.len();
        let mut idx = idx;

        while idx < size {
            let token = &token_list[idx];
            if token.kind == TokenKind::RBrace || token.kind == TokenKind::Eof {
                return ParseResult {
                    value: Some(Value::Object(result)),
                    index: idx + 1,
                };
            }

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
                expect_comma = false;
            }

            if token.kind == TokenKind::String {
                let key = token.value.clone().unwrap_or_default();
                idx += 1;
                if idx < size && token_list[idx].kind == TokenKind::Colon {
                    idx += 1;
                    let pr = self.parse(token_list, idx);
                    if let Some(value) = pr.value {
                        result.insert(key, value);
                    }
                    idx = pr.index;
                    expect_comma = true;
                } else {
                    idx += 1;
                }
            } else {
                idx += 1;
            }
        }

        ParseResult {
            value: Some(Value::Object(result)),
            index: idx,
        }
    }

    fn parse_array(&self, token_list: &[Token], idx: usize) -> ParseResult {
        let mut result = Vec::new();
        let mut expect_comma = false;
        let size = token_list.len();
        let mut idx = idx;

        while idx < size {
            let token = &token_list[idx];
            if token.kind == TokenKind::RBracket || token.kind == TokenKind::Eof {
                return ParseResult {
                    value: Some(Value::Array(result)),
                    index: idx + 1,
                };
            }

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
                } else {
                    idx += 1;
                    continue;
                }
            }

            let valid_token = matches!(token.kind,
                TokenKind::LBrace | TokenKind::LBracket | TokenKind::String | 
                TokenKind::Number | TokenKind::True | TokenKind::False | TokenKind::Null
            );

            if !valid_token {
                return ParseResult {
                    value: Some(Value::Array(result)),
                    index: idx,
                };
            }

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

pub struct JSONSerializer;

impl JSONSerializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, value: &Value) -> String {
        serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
    }

    pub fn serialize_pretty(&self, value: &Value) -> String {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "null".to_string())
    }
}

pub struct JSONAutoCorrector {
    tokenizer: Arc<JSONTokenizer>,
    parser: Arc<JSONParser>,
    serializer: Arc<JSONSerializer>,
}

impl JSONAutoCorrector {
    pub fn new() -> Self {
        Self {
            tokenizer: Arc::new(JSONTokenizer::new()),
            parser: Arc::new(JSONParser::new()),
            serializer: Arc::new(JSONSerializer::new()),
        }
    }

    pub fn autocorrect(&self, input: &str) -> String {
        let tokens = self.tokenizer.tokenize(input);
        let fixed_tokens = JSONTokenFixer::fix_tokens(tokens);
        let parse_result = self.parser.parse(&fixed_tokens, 0);

        let result = match parse_result.value {
            Some(value) => value,
            None => Value::Object(Map::new()),
        };

        self.serializer.serialize(&result)
    }
}

#[no_mangle]
pub extern "system" fn Java_com_github_dedinc_jsonfixer4j_JSONFixerRust_autocorrect(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {

    let input: String = env
        .get_string(&input)
        .expect("Couldn't get Java string!")
        .into();

    let corrector = JSONAutoCorrector::new();
    let result = corrector.autocorrect(&input);

    env.new_string(result)
        .expect("Couldn't create Java string!")
        .into_raw()
}

#[derive(Debug)]
pub struct ParseResult {
    pub value: Option<Value>,
    pub index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let tokenizer = JSONTokenizer::new();
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
    fn test_autocorrector() {
        let corrector = JSONAutoCorrector::new();

        let result = corrector.autocorrect("{\"key\":42");
        assert_eq!(result, "{\"key\":42}");

        let result = corrector.autocorrect("{\"key\":[1,2,3}");
        assert_eq!(result, "{\"key\":[1,2,3]}");

        let result = corrector.autocorrect("{\"key1\":42 \"key2\":true}");
        assert_eq!(result, "{\"key1\":42, \"key2\":true}");
    }
}
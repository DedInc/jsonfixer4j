use crate::parser::JSONParser;
use crate::serializer::JSONSerializer;
use crate::token_fixer::JSONTokenFixer;
use crate::tokenizer::JSONTokenizer;
use serde_json::{Map, Value};

/// Main JSON auto-correction engine
/// Combines tokenization, fixing, parsing, and serialization
pub struct JSONAutoCorrector {
    tokenizer: JSONTokenizer,
    parser: JSONParser,
    serializer: JSONSerializer,
}

impl JSONAutoCorrector {
    pub fn new() -> Self {
        Self {
            tokenizer: JSONTokenizer::new(),
            parser: JSONParser::new(),
            serializer: JSONSerializer::new(),
        }
    }

    /// Auto-correct broken JSON string
    /// This is the main entry point for JSON correction
    pub fn autocorrect(&mut self, input: &str) -> String {
        // Step 1: Tokenize the input
        let tokens = self.tokenizer.tokenize(input);

        // Step 2: Fix token stream (add missing brackets, etc.)
        let fixed_tokens = JSONTokenFixer::fix_tokens(tokens);

        // Step 3: Parse tokens into JSON value
        let parse_result = self.parser.parse(&fixed_tokens, 0);

        // Step 4: Get the result value or default to empty object
        let result = match parse_result.value {
            Some(value) => value,
            None => Value::Object(Map::new()),
        };

        // Step 5: Serialize back to JSON string
        self.serializer.serialize(&result)
    }

    /// Auto-correct and return pretty-printed JSON
    pub fn autocorrect_pretty(&mut self, input: &str) -> String {
        let tokens = self.tokenizer.tokenize(input);
        let fixed_tokens = JSONTokenFixer::fix_tokens(tokens);
        let parse_result = self.parser.parse(&fixed_tokens, 0);

        let result = match parse_result.value {
            Some(value) => value,
            None => Value::Object(Map::new()),
        };

        self.serializer.serialize_pretty(&result)
    }
}

impl Default for JSONAutoCorrector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocorrect_missing_brace() {
        let mut corrector = JSONAutoCorrector::new();
        let result = corrector.autocorrect(r#"{"key":42"#);
        assert_eq!(result, r#"{"key":42}"#);
    }

    #[test]
    fn test_autocorrect_missing_bracket() {
        let mut corrector = JSONAutoCorrector::new();
        let result = corrector.autocorrect(r#"{"key":[1,2,3}"#);
        assert_eq!(result, r#"{"key":[1,2,3]}"#);
    }

    #[test]
    fn test_autocorrect_missing_comma() {
        let mut corrector = JSONAutoCorrector::new();
        let result = corrector.autocorrect(r#"{"key1":42 "key2":true}"#);
        assert_eq!(result, r#"{"key1":42,"key2":true}"#);
    }

    #[test]
    fn test_autocorrect_incomplete_string() {
        let mut corrector = JSONAutoCorrector::new();
        let result = corrector.autocorrect(r#"{"title":"Hello"#);
        assert_eq!(result, r#"{"title":"Hello"}"#);
    }

    #[test]
    fn test_autocorrect_partial_literals() {
        let mut corrector = JSONAutoCorrector::new();
        
        let result = corrector.autocorrect(r#"{"flag":tr}"#);
        assert_eq!(result, r#"{"flag":true}"#);
        
        let result = corrector.autocorrect(r#"{"flag":fals}"#);
        assert_eq!(result, r#"{"flag":false}"#);
        
        let result = corrector.autocorrect(r#"{"value":nul}"#);
        assert_eq!(result, r#"{"value":null}"#);
    }

    #[test]
    fn test_autocorrect_trailing_comma() {
        let mut corrector = JSONAutoCorrector::new();
        let result = corrector.autocorrect(r#"{"key1":1,"key2":2,}"#);
        assert_eq!(result, r#"{"key1":1,"key2":2}"#);
    }

    #[test]
    fn test_autocorrect_nested_incomplete() {
        let mut corrector = JSONAutoCorrector::new();
        let result = corrector.autocorrect(r#"{"outer":{"inner":[1,2,3"#);
        assert_eq!(result, r#"{"outer":{"inner":[1,2,3]}}"#);
    }
}


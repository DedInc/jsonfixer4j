//! # JSONFixer4J - High-Performance JSON Auto-Correction Library
//!
//! A fast and efficient Rust library for automatically correcting broken JSON strings.
//! Optimized for handling large JSON objects (megabytes) with incomplete sequences,
//! missing brackets, braces, and other common JSON errors.
//!
//! ## Features
//! - Fast tokenization using byte-level processing
//! - Automatic bracket/brace matching and correction
//! - Handles incomplete strings, literals, and numbers
//! - Memory-efficient parsing for large JSON objects
//! - JNI bindings for Java integration

mod autocorrector;
mod parser;
mod serializer;
mod token;
mod token_fixer;
mod tokenizer;

// Re-export main types
pub use autocorrector::JSONAutoCorrector;
pub use parser::{JSONParser, ParseResult};
pub use serializer::JSONSerializer;
pub use token::{Token, TokenKind};
pub use token_fixer::JSONTokenFixer;
pub use tokenizer::JSONTokenizer;

// JNI bindings
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::sync::Mutex;

// Thread-local corrector for better performance in multi-threaded environments
use once_cell::sync::Lazy;

static CORRECTOR: Lazy<Mutex<JSONAutoCorrector>> = Lazy::new(|| {
    Mutex::new(JSONAutoCorrector::new())
});

/// JNI entry point for Java integration
/// Optimized to reuse corrector instance for better performance

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

    // Use thread-safe shared corrector for better performance
    let mut corrector = CORRECTOR.lock().unwrap();
    let result = corrector.autocorrect(&input);

    env.new_string(result)
        .expect("Couldn't create Java string!")
        .into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenizer() {
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
    fn test_basic_autocorrect() {
        let mut corrector = JSONAutoCorrector::new();

        let result = corrector.autocorrect("{\"key\":42");
        assert_eq!(result, r#"{"key":42}"#);

        let result = corrector.autocorrect("{\"key\":[1,2,3}");
        assert_eq!(result, r#"{"key":[1,2,3]}"#);

        let result = corrector.autocorrect(r#"{"key1":42 "key2":true}"#);
        assert_eq!(result, r#"{"key1":42,"key2":true}"#);
    }
}
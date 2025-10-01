use serde_json::Value;

/// JSON serializer for converting parsed values back to strings
pub struct JSONSerializer;

impl JSONSerializer {
    pub fn new() -> Self {
        Self {}
    }

    /// Serialize JSON value to compact string
    #[inline]
    pub fn serialize(&self, value: &Value) -> String {
        serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
    }

    /// Serialize JSON value to pretty-printed string
    #[inline]
    pub fn serialize_pretty(&self, value: &Value) -> String {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "null".to_string())
    }
}

impl Default for JSONSerializer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serialize() {
        let serializer = JSONSerializer::new();
        let value = json!({"key": 42});
        let result = serializer.serialize(&value);
        assert_eq!(result, r#"{"key":42}"#);
    }

    #[test]
    fn test_serialize_pretty() {
        let serializer = JSONSerializer::new();
        let value = json!({"key": 42});
        let result = serializer.serialize_pretty(&value);
        assert!(result.contains("  ")); // Should have indentation
    }
}


//! Normalize historical tool arguments to a JSON object string.
use serde_json::{Value, json};
/// Encode historical tool arguments as a JSON object string.
///
/// # Arguments
/// * arguments - Object, scalar, or malformed historical argument text.
///
/// # Returns
/// A serialized object, recovered primitive fields, or an input wrapper.
///
/// # Examples
///
///     let result = codetether_provider_zai::tool_arguments::normalize("42");
///     assert_eq!(serde_json::from_str::<serde_json::Value>(&result).unwrap(),
///         serde_json::json!({"input": 42}));
pub fn normalize(arguments: &str) -> String {
    // The live Z.AI endpoint rejects object-typed historical arguments in
    // assistant.tool_calls and accepts OpenAI-style JSON strings instead.
    if let Ok(parsed) = serde_json::from_str::<Value>(arguments) {
        if parsed.is_object() {
            return serde_json::to_string(&parsed).unwrap_or_else(|_| "{}".to_string());
        }
        return json!({"input": parsed}).to_string();
    }

    if let Some(salvaged) = super::salvage::salvage_json_object(arguments) {
        return serde_json::to_string(&salvaged).unwrap_or_else(|_| "{}".to_string());
    }

    json!({"input": arguments}).to_string()
}

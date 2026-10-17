//! Recover primitive fields from a malformed JSON object.
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
/// Recover recognizable primitive fields from object-like argument text.
///
/// # Arguments
/// * arguments - Text to inspect using the existing conservative pattern.
///
/// # Returns
/// An object with recovered fields, or None when no fields are found.
///
/// # Panics
/// Panics if the built-in regular expression is invalid.
///
/// # Examples
///
///     use codetether_provider_zai::tool_arguments::salvage_json_object;
///     assert!(salvage_json_object("not an object").is_none());
pub fn salvage_json_object(arguments: &str) -> Option<Value> {
    let trimmed = arguments.trim();
    if !trimmed.starts_with('{') {
        return None;
    }

    static RE_SIMPLE_PAIR: Lazy<Regex> = Lazy::new(|| {
        // Matches simple JSON key/value pairs where the value is a primitive
        // or a quoted string. This is intentionally conservative.
        Regex::new(
                r#"(?s)\"(?P<k>[^\"\\]*(?:\\.[^\"\\]*)*)\"\s*:\s*(?P<v>\"(?:\\.|[^\"])*\"|true|false|null|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)"#,
            )
            .expect("invalid regex")
    });

    let mut map = serde_json::Map::new();
    for caps in RE_SIMPLE_PAIR.captures_iter(trimmed) {
        let key = caps.name("k")?.as_str();
        let val_str = caps.name("v")?.as_str();
        if let Ok(val) = serde_json::from_str::<Value>(val_str) {
            map.insert(key.to_string(), val);
        }
    }

    if map.is_empty() {
        None
    } else {
        Some(Value::Object(map))
    }
}

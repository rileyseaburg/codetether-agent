//! Optional runtime fields preserve existing values.
use serde_json::{Map, Value, json};

/// Insert a string field into a JSON object only when the value is present.
///
/// # Examples
///
/// ```rust
/// use codetether_runtime_input::insert_field;
/// use serde_json::{Map, Value};
///
/// let mut obj = Map::<String, Value>::new();
/// insert_field(&mut obj, "example", Some("value"));
/// insert_field(&mut obj, "missing", None);
///
/// assert_eq!(obj["example"], "value");
/// assert!(!obj.contains_key("missing"));
/// ```
/// # Arguments
/// Receives an object, key and optional value.
/// # Returns
/// Updates the object only when the value is present and the key is absent.
pub fn insert_field(obj: &mut Map<String, Value>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        obj.entry(key.to_string()).or_insert_with(|| json!(value));
    }
}

//! Default function and JSON values used by request deserialization.

pub(crate) fn default_function_type() -> String {
    "function".to_string()
}

pub(crate) fn default_json_object() -> serde_json::Value {
    serde_json::json!({})
}

pub(crate) fn default_json_object_string() -> String {
    "{}".to_string()
}

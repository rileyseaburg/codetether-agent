//! Inject a tool-call correlation ID into object inputs.

/// Inject the progress `_tool_call_id` into a tool's input object.
/// # Arguments
/// Input JSON and an optional current tool-call ID.
/// # Returns
/// A cloned input with the ID inserted for JSON objects.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_reporting::progress_input(&serde_json::json!({}), Some("id"))["_tool_call_id"], "id");
/// ```
pub fn progress_input(input: &serde_json::Value, id: Option<&str>) -> serde_json::Value {
    let Some(id) = id else { return input.clone() };
    let mut input = input.clone();
    if let serde_json::Value::Object(map) = &mut input {
        map.insert(
            "_tool_call_id".into(),
            serde_json::Value::String(id.to_string()),
        );
    }
    input
}

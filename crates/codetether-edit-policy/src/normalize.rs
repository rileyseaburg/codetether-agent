//! Normalize the tool name before applying ordered input-shape conversions.
use serde_json::Value;

/// # Arguments
/// Receives a tool name and parsed input to clone.
/// # Returns
/// The normalized tool name and input, preserving existing canonical fields.
/// # Examples
/// ```
/// let (name, input) = codetether_edit_policy::normalize_tool_call_for_execution("advanced_edit", &serde_json::json!({"filePath":"a.rs"}));
/// assert_eq!(name, "edit");
/// assert_eq!(input["path"], "a.rs");
/// ```
pub fn normalize_tool_call_for_execution(tool_name: &str, tool_input: &Value) -> (String, Value) {
    let mut normalized_name = tool_name.to_string();
    let mut normalized_input = tool_input.clone();

    if tool_name == "advanced_edit" {
        normalized_name = "edit".to_string();
    }

    super::normalize_single::apply(&normalized_name, &mut normalized_input);
    super::normalize_multi::apply(&normalized_name, &mut normalized_input);

    (normalized_name, normalized_input)
}

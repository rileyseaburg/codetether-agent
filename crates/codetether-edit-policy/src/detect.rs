//! Inspect replacement payloads rather than unrelated tool fields.
use super::marker::stub_marker_in_text;
use serde_json::Value;

/// # Arguments
/// Receives a tool name and parsed input.
/// # Returns
/// The first matching stub reason in replacement content, or None.
/// # Examples
/// ```
/// assert!(codetether_edit_policy::detect_stub_in_tool_input("write", &serde_json::json!({"content":"TODO"})).is_some());
/// ```
pub fn detect_stub_in_tool_input(tool_name: &str, tool_input: &Value) -> Option<String> {
    let check = |label: &str, text: &str| {
        stub_marker_in_text(text).map(|marker| format!("{label} contains stub marker \"{marker}\""))
    };

    match tool_name {
        "write" => tool_input
            .get("content")
            .and_then(Value::as_str)
            .and_then(|text| check("content", text)),
        "edit" | "confirm_edit" => tool_input
            .get("new_string")
            .or_else(|| tool_input.get("newString"))
            .and_then(Value::as_str)
            .and_then(|text| check("new_string", text)),
        "advanced_edit" => tool_input
            .get("newString")
            .and_then(Value::as_str)
            .and_then(|text| check("newString", text)),
        "multiedit" | "confirm_multiedit" => {
            let edits = tool_input.get("edits").and_then(Value::as_array)?;
            for (idx, edit) in edits.iter().enumerate() {
                if let Some(reason) = edit
                    .get("new_string")
                    .or_else(|| edit.get("newString"))
                    .and_then(Value::as_str)
                    .and_then(|text| check(&format!("edits[{idx}].new_string"), text))
                {
                    return Some(reason);
                }
            }
            None
        }
        "patch" => super::patch::detect(tool_input),
        _ => None,
    }
}

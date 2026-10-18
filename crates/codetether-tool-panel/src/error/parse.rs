use super::StructuredToolError;
use serde_json::Value;

/// Extract structured fields from a tool error envelope.
///
/// # Arguments
/// * `text` - Tool output containing a JSON error envelope.
/// # Returns
/// Parsed fields, or `None` when JSON parsing fails or the error field is absent.
/// # Examples
/// ```
/// assert!(codetether_tool_panel::error::parse_structured_tool_error("invalid").is_none());
/// ```
pub fn parse_structured_tool_error(text: &str) -> Option<StructuredToolError> {
    let value: Value = serde_json::from_str(text).ok()?;
    let error = value.get("error")?;

    let code = error
        .get("code")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let tool = error
        .get("tool")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let message = error
        .get("message")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let missing_fields = error
        .get("missing_fields")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let example = error
        .get("example")
        .and_then(|value| serde_json::to_string_pretty(value).ok());

    Some(StructuredToolError {
        code,
        tool,
        message,
        missing_fields,
        example,
    })
}

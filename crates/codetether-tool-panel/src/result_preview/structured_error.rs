//! Compact rendering of structured tool errors (`{"error": {...}}`).
//!
//! Guard-rail refusals are pretty-printed JSON meant for the model; humans
//! only need the code, the message, and which fields the retry must add.

use serde_json::Value;

/// Summarise a structured error, or `None` when `output` is not one.
pub(super) fn format(output: &str) -> Option<String> {
    let value: Value = serde_json::from_str(output).ok()?;
    let error = value.get("error")?.as_object()?;
    let code = error.get("code")?.as_str()?;
    let message = error.get("message").and_then(Value::as_str).unwrap_or("");
    let mut text = format!("{code}: {message}");
    if let Some(fields) = error.get("missing_fields").and_then(Value::as_array) {
        let names: Vec<&str> = fields.iter().filter_map(Value::as_str).collect();
        if !names.is_empty() {
            text.push_str(&format!("\nneeds: {}", names.join(", ")));
        }
    }
    if let Some(id) = error.get("approval_request_id").and_then(Value::as_str) {
        text.push_str(&format!("\napproval: {id}"));
    }
    Some(text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn summarises_code_message_and_fields() {
        let output = r#"{"error":{"code":"TOOL_JUSTIFICATION_REQUIRED","message":"held",
            "missing_fields":["justification"],"example":{"command":"ls"}}}"#;
        let text = super::format(output).expect("structured");
        assert_eq!(
            text,
            "TOOL_JUSTIFICATION_REQUIRED: held\nneeds: justification"
        );
    }

    #[test]
    fn plain_output_is_not_structured() {
        assert!(super::format("ok").is_none());
        assert!(super::format(r#"{"error":"flat"}"#).is_none());
    }
}

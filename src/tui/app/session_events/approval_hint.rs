//! Approval-required status text for tool completions.

use serde_json::Value;

pub(super) fn status_text(tool: &str, output: &str) -> Option<String> {
    let id = approval_id(output)?;
    let guidance = crate::tui::ui::trust_status::approval_guidance();
    Some(format!(
        "Approval required for `{tool}`. Ctrl+A approves, Ctrl+D denies. Slash: `/approve {id}` or `/deny {id}`. {guidance}"
    ))
}

fn approval_id(output: &str) -> Option<String> {
    let value: Value = serde_json::from_str(output).ok()?;
    let error = value.get("error")?;
    string(error, "approval_request_id").or_else(|| {
        error
            .get("example")
            .and_then(|example| string(example, "approval_id"))
    })
}

fn string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

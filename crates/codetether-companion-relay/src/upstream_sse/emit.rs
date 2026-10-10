//! Interpret one OpenAI-style chunk: text deltas only, no tools or errors.
use anyhow::{Result, bail};
use serde_json::Value;

/// Forward `delta.content` text; reject provider errors and tool calls.
pub(crate) fn emit(value: &Value, delta: &mut dyn FnMut(&str)) -> Result<()> {
    if !value.is_object() || value.get("error").is_some_and(|e| !e.is_null()) {
        bail!("Analysis provider error");
    }
    let choices = value.get("choices").and_then(Value::as_array);
    for choice in choices.into_iter().flatten() {
        if choice.get("finish_reason").and_then(Value::as_str) == Some("error") {
            bail!("Analysis provider error");
        }
        let Some(d) = choice.get("delta").filter(|d| d.is_object()) else {
            continue;
        };
        if d.get("tool_calls").is_some_and(|t| !t.is_null()) {
            bail!("Tools are not permitted for screen analysis");
        }
        if let Some(text) = d.get("content").and_then(Value::as_str) {
            delta(text);
        }
    }
    Ok(())
}

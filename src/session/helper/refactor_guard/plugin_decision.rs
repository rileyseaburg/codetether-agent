use serde_json::Value;

use super::types::GuardViolation;

pub fn read(value: &Value) -> Option<GuardViolation> {
    let value = value.get("ok").unwrap_or(value);
    let parsed = value
        .as_str()
        .and_then(|text| serde_json::from_str::<Value>(text).ok());
    let decision = parsed.as_ref().unwrap_or(value);
    let action = decision
        .get("action")
        .or_else(|| decision.get("status"))?
        .as_str()?;
    if !matches!(action, "continue" | "deny" | "block") {
        return None;
    }
    Some(GuardViolation::new(
        ".codetether/refactor_guard.tether",
        prompt(decision),
    ))
}

fn prompt(decision: &Value) -> &str {
    decision
        .get("prompt")
        .or_else(|| decision.get("message"))
        .and_then(Value::as_str)
        .unwrap_or("TetherScript refactor guard requested continuation.")
}

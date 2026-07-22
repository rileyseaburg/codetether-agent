//! Dependency barriers for calls that cannot safely execute in one batch.

use anyhow::{Result, bail};
use serde_json::Value;

pub(super) fn validate(calls: &[(String, String)]) -> Result<()> {
    if calls.len() < 2 {
        return Ok(());
    }
    if calls
        .iter()
        .any(|(name, arguments)| terminal(name, arguments))
    {
        bail!("goal completion must be the only call in a response and follow real results");
    }
    let has_patch = calls.iter().any(|(name, _)| name == "apply_patch");
    let has_other = calls.iter().any(|(name, _)| name != "apply_patch");
    if has_patch && has_other {
        bail!("apply_patch cannot share a batch with tools that may depend on the patch result");
    }
    Ok(())
}

fn terminal(name: &str, arguments: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(arguments) else {
        return false;
    };
    match name {
        "update_goal" => matches!(
            value.get("status").and_then(Value::as_str),
            Some("complete" | "blocked")
        ),
        "session_task" => value.get("action").and_then(Value::as_str) == Some("complete_goal"),
        _ => false,
    }
}

#[cfg(test)]
#[path = "batch_guard_tests.rs"]
mod tests;

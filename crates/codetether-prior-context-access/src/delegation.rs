//! Delegation actions that cannot yet carry a prior-context ceiling.

use serde_json::Value;

/// Detect actions that can start or contact an unrestricted agent loop.
pub(super) fn crosses_unenforced_boundary(name: &str, args: &Value) -> bool {
    match (name, action(args)) {
        ("go", Some("execute")) | ("ralph", Some("run")) => true,
        ("relay_autochat", Some("delegate" | "handoff")) => true,
        _ => false,
    }
}

fn action(args: &Value) -> Option<&str> {
    args.get("action").and_then(Value::as_str)
}

//! Action-specific network classification for mixed-capability tools.

use serde_json::Value;

pub(super) fn check(tool: &str, args: &Value) -> Option<bool> {
    let action = args.get("action").and_then(Value::as_str);
    match tool {
        "go" => Some(action == Some("execute")),
        "ralph" => Some(action == Some("run")),
        "relay_autochat" => Some(matches!(
            action,
            Some("init" | "delegate" | "handoff" | "complete")
        )),
        "session_recall" => Some(args.get("mode").and_then(Value::as_str) == Some("answer")),
        "mux_control" => Some(true),
        "browserctl" | "edit" | "multiedit" | "image" => None,
        _ => None,
    }
}

#[cfg(test)]
#[path = "network_governed_actions_tests.rs"]
mod tests;

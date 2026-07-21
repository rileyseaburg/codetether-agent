//! Detection of child creation forbidden inside mux sessions.

use serde_json::Value;

pub(super) fn work_action<'a>(tool: &str, input: &'a Value) -> Option<&'a str> {
    (tool == "agent")
        .then(|| input.get("action")?.as_str())
        .flatten()
        .filter(|action| matches!(*action, "ask" | "spawn"))
}

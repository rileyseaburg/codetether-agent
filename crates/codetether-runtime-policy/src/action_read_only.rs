//! Per-action side-effect classification for action-dispatch tools.
//!
//! Tools such as `memory`, `okr`, or `session_task` take an `action` argument
//! and are unclassified as a whole, so `ask` mode prompted even for a pure
//! `list`. This allowlists the actions that only read state, plus
//! session-local work-item tracking that never leaves the session's own files.

use serde_json::Value;

/// `(tool, actions)` pairs whose listed actions only read state.
const READ_ACTIONS: &[(&str, &[&str])] = &[
    ("memory", &["search", "get", "list", "tags", "stats"]),
    (
        "okr",
        &[
            "get_okr",
            "list_okrs",
            "query_okrs",
            "get_run",
            "list_runs",
            "query_runs",
            "stats",
        ],
    ),
    ("task", &["status", "list"]),
    (
        "swarm_share",
        &["get", "query_tags", "query_prefix", "list"],
    ),
    ("ralph", &["status"]),
    ("go", &["watch", "status"]),
];

/// Tools that only mutate the current session's own work-item log.
const SESSION_BOOKKEEPING: &[&str] = &["session_task"];

/// Whether this invocation only reads state or updates session bookkeeping.
pub(super) fn allowed(tool_name: &str, args: &Value) -> bool {
    if SESSION_BOOKKEEPING.contains(&tool_name) {
        return true;
    }
    let Some(action) = args.get("action").and_then(Value::as_str) else {
        return false;
    };
    READ_ACTIONS
        .iter()
        .any(|(tool, actions)| *tool == tool_name && actions.contains(&action.trim()))
}

#[cfg(test)]
#[path = "action_read_only_tests.rs"]
mod tests;

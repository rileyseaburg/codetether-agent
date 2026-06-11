use super::args::translate;
use serde_json::json;

#[test]
fn translates_timeout_ms_to_bash_seconds() {
    let args = translate(&json!({
        "command": "pwd",
        "cwd": "/tmp",
        "timeout_ms": 1500
    }))
    .expect("args");

    assert_eq!(args["command"], "pwd");
    assert_eq!(args["cwd"], "/tmp");
    assert_eq!(args["timeout"], 2);
}

#[test]
fn requires_command() {
    assert!(translate(&json!({})).is_err());
}

#[test]
fn preserves_approval_and_prefix_metadata() {
    let args = translate(&json!({
        "command": "cargo test",
        "approval_id": "approval-1",
        "prefix_rule": ["cargo", "test"]
    }))
    .expect("args");

    assert_eq!(args["approval_id"], "approval-1");
    assert_eq!(args["prefix_rule"][1], "test");
}

use crate::approval::{ApprovalStore, session_command_grants};
use crate::config::Config;
use serde_json::json;

#[path = "session_prefix_scope_support.rs"]
mod support;

#[test]
fn command_prefix_grant_does_not_cross_session_or_workspace() {
    let _scope = support::Scope::new();
    let args = json!({
        "command": "cargo test --lib first",
        "cwd": "workspace-a",
        "prefix_rule": ["cargo", "test"],
        "__ct_session_id": "session-a"
    });
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(),
        "bash",
        &args,
    )
    .expect("approval required");
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    ApprovalStore::open_default()
        .unwrap()
        .approve(id, "test", "session approval")
        .unwrap();
    session_command_grants::grant_for_request(id);

    let same = json!({
        "command": "cargo test --lib next",
        "cwd": "workspace-a",
        "__ct_session_id": "session-a",
    });
    assert!(allowed(&same));
    let session =
        json!({"command": "cargo test", "cwd": "workspace-a", "__ct_session_id": "session-b"});
    assert!(!allowed(&session));
    let workspace =
        json!({"command": "cargo test", "cwd": "workspace-b", "__ct_session_id": "session-a"});
    assert!(!allowed(&workspace));
}

fn allowed(args: &serde_json::Value) -> bool {
    crate::runtime_policy::evaluate_tool_invocation_with_config(&Config::default(), "bash", args)
        .is_none()
}

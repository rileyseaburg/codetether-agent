//! Only approval/correlation metadata is excluded from invocation identity.

use super::for_tool;
use serde_json::json;

#[test]
fn approval_scope_is_stable_across_progress_injection() {
    let args = json!({"cmd": "cargo update", "sandbox_permissions": "require_escalated"});
    let mut execution = args.clone();
    execution["approval_id"] = json!("approved-request");
    execution["justification"] = json!("same request, new explanation");
    execution["_tool_call_id"] = json!("runtime-progress-id");
    assert_eq!(
        for_tool("exec_command", &args).resource,
        for_tool("exec_command", &execution).resource
    );
}

#[test]
fn approval_scope_still_binds_command_workspace_and_escalation() {
    let args = json!({"cmd": "cargo test", "workdir": "/workspace",
        "sandbox_permissions": "use_default"});
    for (field, value) in [
        ("cmd", "cargo update"),
        ("workdir", "/elsewhere"),
        ("sandbox_permissions", "require_escalated"),
        ("_unknown", "not-correlation"),
    ] {
        let mut changed = args.clone();
        changed[field] = json!(value);
        assert_ne!(
            for_tool("exec_command", &args).resource,
            for_tool("exec_command", &changed).resource,
            "{field} must remain approval-bound"
        );
    }
}

#[test]
fn progress_metadata_does_not_widen_tool_identity() {
    let args = json!({"cmd": "cargo test", "_tool_call_id": "same-id"});
    assert_ne!(
        for_tool("exec_command", &args).resource,
        for_tool("bash", &args).resource
    );
}

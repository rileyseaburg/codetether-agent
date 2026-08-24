use serde_json::json;

#[test]
fn patch_detail_is_not_truncated() {
    let patch = format!("--- a/file\n+++ b/file\n{}", "+full line\n".repeat(100));
    assert_eq!(
        super::render("apply_patch", &json!({"patch": patch})),
        Some(patch)
    );
}

#[test]
fn escalated_command_detail_displays_signed_authority() {
    let mut args = json!({
        "cmd": "run",
        "workdir": "/workspace",
        "sandbox_permissions": "require_escalated",
        "__ct_effective_network_allowed": true,
        "__ct_parent_workspace": "/workspace",
        "__ct_session_id": "approval-detail-test",
    });
    crate::tool::network_access::bind_trusted(&mut args, true);
    let detail = super::render("exec_command", &args).expect("detail");
    assert!(detail.contains("command: run"));
    assert!(detail.contains("working directory: /workspace"));
    assert!(detail.contains("network: enabled"));
    assert!(detail.contains("sandbox permissions: require_escalated"));
}

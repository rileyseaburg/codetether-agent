use crate::tool::Tool;

#[test]
fn schema_is_mux_only_and_supports_watch() {
    let schema = super::MuxControlTool.parameters();
    let actions = schema["properties"]["action"]["enum"].as_array().unwrap();
    assert!(actions.iter().any(|item| item == "watch"));
    assert!(actions.iter().any(|item| item == "roll"));
    assert!(!actions.iter().any(|item| item == "spawn"));
}

#[tokio::test]
async fn lifecycle_workspace_cannot_escape_trusted_parent() {
    let parent = tempfile::tempdir().expect("parent");
    let outside = tempfile::tempdir().expect("outside");
    let mut args = serde_json::json!({
            "action": "start",
            "name": "scope-test",
            "workspace": outside.path(),
            "no_worktree": true,
            "__ct_parent_workspace": parent.path(),
            "__ct_session_id": "mux-workspace-test",
        });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let result = super::MuxControlTool
        .execute(args)
        .await;
    assert!(
        result
            .expect_err("outside workspace")
            .to_string()
            .contains("outside trusted parent")
    );
}
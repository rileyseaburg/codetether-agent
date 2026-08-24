use super::result;
use serde_json::json;

fn scoped(root: &std::path::Path, key: &str, target: &std::path::Path) -> serde_json::Value {
    let mut args = json!({
        key: target,
        "__ct_parent_workspace": root,
        "__ct_session_id": "workdir-test",
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    args
}
#[test]
fn command_workdir_must_remain_in_parent_workspace() {
    let root = tempfile::tempdir().expect("root");
    let inside = root.path().join("inside");
    std::fs::create_dir(&inside).expect("inside");
    let outside = tempfile::tempdir().expect("outside");

    let inside_args = scoped(root.path(), "cwd", &inside);
    assert!(result("bash", &inside_args).is_none());
    let outside_args = scoped(root.path(), "workdir", outside.path());
    let denied = result("exec_command", &outside_args).expect("outside path denied");
    assert!(!denied.success);
    assert_eq!(denied.metadata["error_code"], "COMMAND_WORKDIR_DENIED");
}

#[test]
fn direct_api_without_session_workspace_retains_explicit_target() {
    let args = json!({"cwd": "/"});
    assert!(result("bash", &args).is_none());
}
use super::super::{BashTool, Tool};
use codetether_config::{AccessMode, Config};
use codetether_native_approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use serde_json::json;

#[tokio::test]
async fn approved_bash_keeps_sandbox_or_uses_approved_fallback() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "command": "printf ok > approved.txt",
        "cwd": data.path().display().to_string(),
        "justification": "write the approval marker"
    });
    let blocked = codetether_native_runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(),
        "bash",
        &args,
    )
    .expect("approval required");
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    ApprovalStore::open(data.path().join("approvals"))
        .unwrap()
        .approve(id, "riley", "ok")
        .unwrap();
    args["approval_id"] = json!(id);

    let tool = BashTool::configured(10, true, None);
    let result = tool.execute(args).await.unwrap();

    if codetether_sandbox_runtime::unavailable_reason().is_some() {
        assert!(result.success);
        assert_eq!(result.metadata.get("sandboxed"), Some(&json!(false)));
        assert_eq!(
            result.metadata.get("unsafe_fallback_reason"),
            Some(&json!("approved_os_sandbox_unavailable_fallback"))
        );
    } else {
        assert_eq!(result.metadata.get("sandboxed"), Some(&json!(true)));
    }
}

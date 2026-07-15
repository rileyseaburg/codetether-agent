use super::super::{BashTool, Tool};
use crate::approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use crate::config::{AccessMode, Config};
use serde_json::json;

#[tokio::test]
async fn approved_bash_keeps_sandbox_or_uses_approved_fallback() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "command": "printf ok > approved.txt",
        "cwd": data.path().display().to_string()
    });
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
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

    let tool = BashTool {
        timeout_secs: 10,
        sandboxed: true,
        default_cwd: None,
    };
    let result = tool.execute(args).await.unwrap();

    if crate::tool::sandbox::unavailable_reason().is_some() {
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

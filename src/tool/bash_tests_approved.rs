use super::super::{BashTool, Tool};
use crate::approval::{ApprovalStore, test_env::ENV_LOCK};
use crate::config::Config;
use serde_json::json;

struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[tokio::test]
async fn approved_bash_keeps_sandbox_or_uses_approved_fallback() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let _env = EnvGuard;
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

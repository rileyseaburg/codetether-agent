use super::Registry;
use crate::tool::exec_command::ExecCommandTool;
use crate::tool::{Tool, ToolRegistry};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn exec_command_returns_completed_output_and_metadata() {
    let directory = tempfile::tempdir().unwrap();
    let tool = ExecCommandTool::new(
        Arc::new(Registry::default()),
        Some(directory.path().to_path_buf()),
    );
    let result = tool
        .execute(json!({"cmd": "pwd -P", "yield_time_ms": 250}))
        .await
        .unwrap();
    assert!(result.success);
    assert!(result.output.contains(directory.path().to_str().unwrap()));
    assert_eq!(result.metadata["running"], json!(false));
    assert_eq!(result.metadata["exit_code"], json!(0));
}

#[tokio::test]
async fn exec_command_injects_session_runtime_context() {
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);
    let result = tool
        .execute(json!({
            "cmd": "printenv CODETETHER_SESSION_ID",
            "__ct_session_id": "session-context-test",
            "yield_time_ms": 250
        }))
        .await
        .unwrap();
    assert!(result.success);
    assert!(result.output.contains("session-context-test"));
}

#[test]
fn default_registry_exposes_unified_command_pair() {
    let registry = ToolRegistry::with_defaults();
    assert!(registry.contains("exec_command"));
    assert!(registry.contains("write_stdin"));
}

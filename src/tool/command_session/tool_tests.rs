use super::Registry;
use crate::tool::ToolRegistry;
use crate::tool::exec_command::ExecCommandTool;
#[path = "tool_test_completion.rs"]
mod completion;
use completion::completed;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn exec_command_returns_completed_output_and_metadata() {
    let directory = tempfile::tempdir().unwrap();
    let sessions = Arc::new(Registry::default());
    let tool = ExecCommandTool::new(sessions.clone(), Some(directory.path().to_path_buf()));
    let result = completed(
        tool,
        sessions,
        json!({"cmd": "pwd -P", "yield_time_ms": 250}),
    )
    .await;
    assert!(result.success);
    assert!(result.output.contains(directory.path().to_str().unwrap()));
    assert_eq!(result.metadata["running"], json!(false));
    assert_eq!(result.metadata["exit_code"], json!(0));
}

#[tokio::test]
async fn exec_command_injects_session_runtime_context() {
    let sessions = Arc::new(Registry::default());
    let tool = ExecCommandTool::new(sessions.clone(), None);
    let result = completed(
        tool,
        sessions,
        json!({
            "cmd": "printenv CODETETHER_SESSION_ID",
            "__ct_session_id": "session-context-test",
            "yield_time_ms": 250
        }),
    )
    .await;
    assert!(result.success);
    assert!(result.output.contains("session-context-test"));
}

#[test]
fn default_registry_exposes_unified_command_pair() {
    let registry = ToolRegistry::with_defaults();
    assert!(registry.contains("exec_command"));
    assert!(registry.contains("write_stdin"));
}

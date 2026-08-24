use super::execute;
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use crate::tool::ToolRegistry;
use serde_json::json;

#[tokio::test(flavor = "current_thread")]
async fn nested_mutating_tool_is_rejected_before_dispatch() {
    let _guard = lock_env();
    let data = tempfile::tempdir().expect("data dir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let registry = ToolRegistry::with_defaults_arc();
    let (_, tool_id, result) = execute(
        0,
        "bash".to_string(),
        json!({"command": "echo should-not-run"}),
        registry,
    )
    .await;
    assert_eq!(tool_id, "bash");
    assert!(!result.success);
    assert!(result.output.contains("only accepts read-only"));
}

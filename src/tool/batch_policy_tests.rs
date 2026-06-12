use super::execute;
use crate::approval::test_env::lock_env;
use crate::tool::ToolRegistry;
use serde_json::json;

#[tokio::test(flavor = "current_thread")]
async fn nested_mutating_tool_requires_policy_approval() {
    let _guard = lock_env();
    let data = tempfile::tempdir().expect("data dir");
    // SAFETY: this focused test serializes process env access with the shared env lock.
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let registry = ToolRegistry::with_defaults_arc();
    let (_, tool_id, result) = execute(
        0,
        "bash".to_string(),
        json!({"command": "echo should-not-run"}),
        registry,
    )
    .await;
    // SAFETY: paired with the guarded set_var above.
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    assert_eq!(tool_id, "bash");
    assert!(!result.success);
    assert_eq!(result.metadata["policy_outcome"], "require_approval");
}

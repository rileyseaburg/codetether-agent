//! Existing no-thinker dispatch regression.
use super::*;
use crate::tests::TestRegistry as ToolRegistry;
#[tokio::test]
async fn execute_without_thinker_returns_empty() {
    let registry = Arc::new(ToolRegistry::new());
    let result =
        execute_tool_requests(None, &registry, "p1", "some thought", &["Bash".to_string()]).await;
    assert!(result.is_empty());
}

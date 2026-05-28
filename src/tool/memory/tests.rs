use super::*;
use std::sync::atomic::Ordering;

fn initialized_tool() -> MemoryTool {
    let tool = MemoryTool::new();
    tool.initialized.store(true, Ordering::SeqCst);
    tool
}

#[tokio::test]
async fn test_memory_save_and_get() {
    let tool = initialized_tool();
    let result = tool
        .execute(json!({
            "action": "save", "content": "Test memory content",
            "tags": ["test", "example"], "importance": 4, "scope": "test"
        }))
        .await
        .unwrap();
    assert!(result.success);

    let result = tool
        .execute(json!({"action": "list", "limit": 5, "scope": "test"}))
        .await
        .unwrap();
    assert!(result.output.contains("Test memory content"));

    let result = tool.execute(json!({"action": "stats"})).await.unwrap();
    assert!(result.output.contains("Total entries: 1"));
}

#[tokio::test]
async fn test_memory_search() {
    let tool = initialized_tool();
    for (content, tag) in [
        ("Rust programming insights", "rust"),
        ("Python tips", "python"),
    ] {
        tool.execute(json!({
            "action": "save", "content": content,
            "tags": [tag, "programming"], "scope": "test"
        }))
        .await
        .unwrap();
    }

    let result = tool
        .execute(json!({"action": "search", "tags": ["rust"], "scope": "test"}))
        .await
        .unwrap();
    assert!(result.output.contains("Rust"));
    assert!(!result.output.contains("Python"));
}

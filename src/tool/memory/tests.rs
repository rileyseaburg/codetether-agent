use super::*;
#[path = "tests_support.rs"]
mod tests_support;
use tests_support::*;

#[tokio::test]
async fn test_memory_save_and_get() {
    let _dir = isolate_data_dir();
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
    let _dir = isolate_data_dir();
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

use codetether_agent::tool::{Tool, edit::EditTool};
use serde_json::json;
use tempfile::NamedTempFile;

#[tokio::test]
async fn edit_replace_all_exact_matches() {
    let file = NamedTempFile::new().unwrap();
    tokio::fs::write(file.path(), "one\none\n").await.unwrap();
    let result = EditTool::new()
        .execute(json!({
            "path": file.path(), "old_string": "one", "new_string": "two", "replace_all": true
        }))
        .await
        .unwrap();
    assert!(result.success);
    assert_eq!(result.metadata["replacements"], json!(2));
}

#[tokio::test]
async fn edit_uses_whitespace_tolerant_match() {
    let file = NamedTempFile::new().unwrap();
    tokio::fs::write(file.path(), "fn a() {\n    one();\n}\n")
        .await
        .unwrap();
    let result = EditTool::new()
        .execute(json!({
            "path": file.path(), "old_string": "fn a() {\none();\n}", "new_string": "fn b() {}"
        }))
        .await
        .unwrap();
    assert!(result.success);
    assert_eq!(result.metadata["match_strategy"], json!("whitespace"));
}

#[tokio::test]
async fn edit_not_found_shows_closest_candidate() {
    let file = NamedTempFile::new().unwrap();
    tokio::fs::write(file.path(), "fn close() {\n    call();\n}\n")
        .await
        .unwrap();
    let result = EditTool::new()
        .execute(json!({
            "path": file.path(), "old_string": "fn missing() {\n    call();\n}", "new_string": "x"
        }))
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("closest_candidate"));
}

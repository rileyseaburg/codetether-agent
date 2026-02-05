use codetether_agent::tool::{
    Tool, confirm_edit::ConfirmEditTool, confirm_multiedit::ConfirmMultiEditTool,
};
use serde_json::json;
use tempfile::{NamedTempFile, tempdir};
use tokio::fs;

#[tokio::test]
async fn test_confirm_edit_preview() {
    let tool = ConfirmEditTool::new();
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, "Hello World\nThis is a test")
        .await
        .unwrap();

    let args = json!({
        "path": file_path.to_str().unwrap(),
        "old_string": "Hello World",
        "new_string": "Hello Universe"
    });

    let result = tool.execute(args).await.unwrap();

    assert!(result.success);
    assert!(result.output.contains("Changes require confirmation"));
    assert!(
        result
            .metadata
            .get("requires_confirmation")
            .unwrap()
            .as_bool()
            .unwrap()
    );

    // Verify file wasn't changed
    let content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(content, "Hello World\nThis is a test");
}

#[tokio::test]
async fn test_confirm_edit_apply() {
    let tool = ConfirmEditTool::new();
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, "Hello World\nThis is a test")
        .await
        .unwrap();

    let args = json!({
        "path": file_path.to_str().unwrap(),
        "old_string": "Hello World",
        "new_string": "Hello Universe",
        "confirm": true
    });

    let result = tool.execute(args).await.unwrap();

    assert!(result.success);
    assert!(result.output.contains("✓ Changes applied"));

    // Verify file was changed
    let content = fs::read_to_string(&file_path).await.unwrap();
    assert_eq!(content, "Hello Universe\nThis is a test");
}

#[tokio::test]
async fn test_confirm_multiedit_preview() {
    let tool = ConfirmMultiEditTool::new();
    let dir = tempdir().unwrap();

    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");

    fs::write(&file1, "content 1").await.unwrap();
    fs::write(&file2, "content 2").await.unwrap();

    let args = json!({
        "edits": [
            {
                "file": file1.to_str().unwrap(),
                "old_string": "content 1",
                "new_string": "updated 1"
            },
            {
                "file": file2.to_str().unwrap(),
                "old_string": "content 2",
                "new_string": "updated 2"
            }
        ]
    });

    let result = tool.execute(args).await.unwrap();

    assert!(result.success);
    assert!(
        result
            .output
            .contains("Multi-file changes require confirmation")
    );
    assert!(
        result
            .metadata
            .get("requires_confirmation")
            .unwrap()
            .as_bool()
            .unwrap()
    );

    // Verify files weren't changed
    let content1 = fs::read_to_string(&file1).await.unwrap();
    let content2 = fs::read_to_string(&file2).await.unwrap();
    assert_eq!(content1, "content 1");
    assert_eq!(content2, "content 2");
}

#[tokio::test]
async fn test_confirm_multiedit_reject() {
    let tool = ConfirmMultiEditTool::new();
    let dir = tempdir().unwrap();

    let file = dir.path().join("test.txt");
    fs::write(&file, "original content").await.unwrap();

    let args = json!({
        "edits": [
            {
                "file": file.to_str().unwrap(),
                "old_string": "original content",
                "new_string": "new content"
            }
        ],
        "confirm": false
    });

    let result = tool.execute(args).await.unwrap();

    assert!(result.success);
    assert!(result.output.contains("✗ All changes rejected by user"));

    // Verify file wasn't changed
    let content = fs::read_to_string(&file).await.unwrap();
    assert_eq!(content, "original content");
}

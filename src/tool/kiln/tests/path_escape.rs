use serde_json::json;

use crate::tool::Tool;
use crate::tool::kiln::KilnPluginTool;

#[tokio::test]
async fn rejects_project_file_escape() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("workspace");
    tokio::fs::create_dir_all(&root).await.unwrap();
    let outside = dir.path().join("outside.kl");
    tokio::fs::write(&outside, r#"fn validate() { return Ok("bad") }"#)
        .await
        .unwrap();

    let tool = KilnPluginTool::with_root(root);
    let result = tool
        .execute(json!({
            "path": "../outside.kl",
            "hook": "validate"
        }))
        .await
        .unwrap();

    assert!(!result.success);
    assert!(result.output.contains("escapes workspace root"));
}

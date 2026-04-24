use serde_json::json;

use crate::tool::Tool;
use crate::tool::kiln::KilnPluginTool;

#[tokio::test]
async fn executes_kiln_plugin_from_project_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("feature.kl");
    tokio::fs::write(
        &path,
        r#"fn validate(feature) { return Ok("accepted " + feature) }"#,
    )
    .await
    .unwrap();

    let tool = KilnPluginTool::with_root(dir.path().to_path_buf());
    let result = tool
        .execute(json!({
            "path": "feature.kl",
            "hook": "validate",
            "args": ["rust-project"]
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({"ok": "accepted rust-project"}))
    );
}

use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn executes_tetherscript_plugin_from_project_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("feature.tether");
    tokio::fs::write(
        &path,
        r#"fn validate(feature) { return Ok("accepted " + feature) }"#,
    )
    .await
    .unwrap();

    let tool = TetherScriptPluginTool::with_root(dir.path().to_path_buf());
    let result = tool
        .execute(json!({
            "path": "feature.tether",
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

#[tokio::test]
async fn accepts_legacy_kl_project_file_during_migration() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("legacy.kl");
    tokio::fs::write(&path, r#"fn validate() { return Ok("legacy accepted") }"#)
        .await
        .unwrap();

    let tool = TetherScriptPluginTool::with_root(dir.path().to_path_buf());
    let result = tool
        .execute(json!({
            "path": "legacy.kl",
            "hook": "validate"
        }))
        .await
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.metadata.get("value"),
        Some(&json!({"ok": "legacy accepted"}))
    );
}

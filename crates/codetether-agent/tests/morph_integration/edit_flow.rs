//! Instruction-based edits use the opt-in Morph backend.

use super::{environment, mock, workspace::tempdir};
use codetether_agent::tool::{Tool, edit::EditTool};
use serde_json::{Value, json};
use std::sync::atomic::Ordering;

#[tokio::test]
async fn morph_backed_edit_tool_flow() -> anyhow::Result<()> {
    let _lock = environment::lock().lock().await;
    let dir = tempdir()?;
    let file_path = dir.path().join("sample.txt");
    tokio::fs::write(&file_path, "line-1\nline-2\n").await?;
    let expected = "line-1\nline-2\nline-3\n".to_string();
    let (base_url, requests, _server) = mock::spawn(expected.clone()).await?;
    let _env = environment::enable(&base_url);
    let tool = EditTool::new();
    let result = tool
        .execute(json!({
            "path": file_path.to_string_lossy().to_string(),
            "instruction": "Append line-3",
            "update": "line-3"
        }))
        .await?;
    assert!(result.success, "{}", result.output);
    assert_eq!(
        result
            .metadata
            .get("backend")
            .and_then(Value::as_str)
            .unwrap_or_default(),
        "morph"
    );
    assert!(
        result
            .metadata
            .get("new_string")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .contains("line-3")
    );
    assert_eq!(requests.load(Ordering::SeqCst), 1);
    Ok(())
}

//! Instruction-based multi-edits apply the Morph response to the file.

use super::{environment, mock, workspace::tempdir};
use codetether_agent::tool::{Tool, multiedit::MultiEditTool};
use serde_json::json;
use std::sync::atomic::Ordering;

#[tokio::test]
async fn morph_backed_multiedit_tool_flow() -> anyhow::Result<()> {
    let _lock = environment::lock().lock().await;
    let dir = tempdir()?;
    let file_path = dir.path().join("multi.txt");
    tokio::fs::write(&file_path, "a\n").await?;
    let expected = "a\nb\n".to_string();
    let (base_url, requests, _server) = mock::spawn(expected.clone()).await?;
    let _env = environment::enable(&base_url);
    let tool = MultiEditTool::new();
    let result = tool
        .execute(json!({
            "edits": [{
                "file": file_path.to_string_lossy().to_string(),
                "instruction": "Append b",
                "update": "b"
            }]
        }))
        .await?;
    assert!(result.success, "{}", result.output);
    let updated = tokio::fs::read_to_string(&file_path).await?;
    assert_eq!(updated, expected);
    assert_eq!(requests.load(Ordering::SeqCst), 1);
    Ok(())
}

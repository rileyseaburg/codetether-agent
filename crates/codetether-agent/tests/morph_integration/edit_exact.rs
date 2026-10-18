//! Exact replacements bypass Morph even when its backend is enabled.

use super::{environment, mock, workspace::tempdir};
use codetether_agent::tool::{Tool, edit::EditTool};
use serde_json::json;
use std::sync::atomic::Ordering;

#[tokio::test]
async fn exact_replace_edit_skips_morph_even_when_enabled() -> anyhow::Result<()> {
    let _lock = environment::lock().lock().await;
    let dir = tempdir()?;
    let file_path = dir.path().join("exact-edit.txt");
    tokio::fs::write(&file_path, "alpha\nbeta\n").await?;
    let (base_url, requests, _server) =
        mock::spawn("this should never be returned".to_string()).await?;
    let _env = environment::enable(&base_url);
    let tool = EditTool::new();
    let result = tool
        .execute(json!({
            "path": file_path.to_string_lossy().to_string(),
            "old_string": "beta",
            "new_string": "gamma"
        }))
        .await?;
    assert!(result.success, "{}", result.output);
    assert!(result.metadata.get("backend").is_none());
    assert_eq!(requests.load(Ordering::SeqCst), 0);
    Ok(())
}

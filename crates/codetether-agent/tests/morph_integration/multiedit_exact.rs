//! Exact multi-edits bypass Morph and retain their exact file-content contract.

use super::{environment, mock, workspace::tempdir};
use codetether_agent::tool::{Tool, multiedit::MultiEditTool};
use serde_json::json;
use std::sync::atomic::Ordering;

#[tokio::test]
async fn exact_replace_multiedit_skips_morph_even_when_enabled() -> anyhow::Result<()> {
    let _lock = environment::lock().lock().await;
    let dir = tempdir()?;
    let file_path = dir.path().join("multi-exact.txt");
    tokio::fs::write(&file_path, "a\n").await?;
    let expected = "a\nb\n".to_string();
    let (base_url, requests, _server) =
        mock::spawn("this should never be written".to_string()).await?;
    let _env = environment::enable(&base_url);
    let tool = MultiEditTool::new();
    let result = tool
        .execute(json!({
            "edits": [{
                "file": file_path.to_string_lossy().to_string(),
                "old_string": "a\n",
                "new_string": "a\nb\n"
            }]
        }))
        .await?;
    assert!(result.success, "{}", result.output);
    let updated = tokio::fs::read_to_string(&file_path).await?;
    assert_eq!(updated, expected);
    assert_eq!(requests.load(Ordering::SeqCst), 0);
    Ok(())
}

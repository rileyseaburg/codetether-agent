//! Interpret avatar list models responses.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) async fn read(resp: reqwest::Response) -> Result<ToolResult> {
    let body: Value = resp.json().await.context("Failed to parse response")?;
    let models = body["models"].as_array();

    match models {
        Some(models) if !models.is_empty() => {
            let mut output = format!("Avatar models ({}):\n\n", models.len());
            for m in models {
                let name = m["name"].as_str().unwrap_or("?");
                let path = m["path"].as_str().unwrap_or("?");
                let size = m["size_bytes"].as_u64().unwrap_or(0);
                let size_mb = size as f64 / 1_048_576.0;
                output.push_str(&format!("- {name} ({size_mb:.1}MB)\n  Path: {path}\n"));
            }
            Ok(ToolResult::success(output).with_metadata("count", json!(models.len())))
        }
        _ => Ok(ToolResult::success(
            "No avatar models found. Upload a silent video of yourself \
                 using action 'upload_model' to create an AI clone.",
        )),
    }
}

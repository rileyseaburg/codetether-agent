//! Save generated speech bytes and report job metadata.
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(super) async fn save(resp: reqwest::Response) -> Result<ToolResult> {
    let job_id = resp
        .headers()
        .get("x-job-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Save the WAV to a temp file
    let bytes = resp
        .bytes()
        .await
        .context("Failed to read audio response")?;
    let output_dir = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let output_path = output_dir.join(format!("voice_{job_id}.wav"));
    tokio::fs::write(&output_path, &bytes)
        .await
        .context("Failed to save audio file")?;

    let duration_secs = bytes.len() as f64 / (24000.0 * 2.0); // 24kHz, 16-bit mono

    Ok(ToolResult::success(format!(
            "Generated speech saved to: {}\nJob ID: {job_id}\nApprox duration: {duration_secs:.1}s\nSize: {} bytes",
            output_path.display(),
            bytes.len()
        ))
        .with_metadata("job_id", json!(job_id))
        .with_metadata("output_path", json!(output_path.to_string_lossy()))
        .with_metadata("size_bytes", json!(bytes.len())))
}

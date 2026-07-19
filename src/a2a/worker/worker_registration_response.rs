//! Fail-closed handling of worker registration responses.

use anyhow::{Result, bail};

pub(super) async fn require_success(response: reqwest::Response) -> Result<()> {
    if response.status().is_success() {
        tracing::info!("Worker registered successfully");
        return Ok(());
    }
    let status = response.status();
    let body = response
        .text()
        .await
        .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
    let summary = summarize(&body);
    tracing::warn!(%status, body = %summary, "Failed to register worker");
    bail!("Worker registration rejected with {status}: {summary}")
}

fn summarize(body: &str) -> String {
    const MAX_BODY_CHARS: usize = 512;
    let mut summary = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if summary.chars().count() > MAX_BODY_CHARS {
        summary = summary.chars().take(MAX_BODY_CHARS).collect::<String>();
        summary.push_str("...");
    }
    summary
}

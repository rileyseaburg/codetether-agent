//! Fail-closed handling of worker registration responses.

use anyhow::{Result, bail};

pub async fn require_success(response: reqwest::Response) -> Result<()> {
    if response.status().is_success() {
        tracing::info!(target: "codetether_agent::a2a::worker::worker_security::worker_registration_response", "Worker registered successfully");
        return Ok(());
    }
    let status = response.status();
    let body = response
        .text()
        .await
        .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
    let summary = summarize(&body);
    tracing::warn!(target: "codetether_agent::a2a::worker::worker_security::worker_registration_response", %status, body = %summary, "Failed to register worker");
    bail!("Worker registration rejected with {status}: {summary}")
}

use super::summarize_response_body as summarize;

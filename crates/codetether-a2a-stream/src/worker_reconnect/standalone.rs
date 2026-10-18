use super::Worker;
use crate::ingest::StreamDisconnectReason;
use anyhow::Result;
/// Reconnect a standalone worker after each stream disconnect.
///
/// # Arguments
/// * `host` — Native registration, task, heartbeat and stream services.
/// # Returns
/// Runs until cancelled, retaining a five-second delay between attempts.
/// # Errors
/// Registration, fetch and connection failures are logged; the loop retries.
/// # Examples
/// ```ignore
/// run_worker_loop(&native_worker).await?;
/// ```
pub async fn run_worker_loop<H: Worker>(host: &H) -> Result<()> {
    loop {
        let codebases = host.codebases().await;
        if let Err(error) = host.register().await {
            tracing::warn!(error = %error, "Failed to re-register worker on reconnection");
        }
        if let Err(error) = host.fetch_pending().await {
            tracing::warn!(error = %error, "Reconnect task fetch failed");
        }
        let heartbeat = host.heartbeat();
        match host.connect(&codebases, None).await {
            Ok(StreamDisconnectReason::Ended) => tracing::warn!("Stream ended, reconnecting..."),
            Ok(StreamDisconnectReason::ReadError(error)) => {
                tracing::warn!(error = %error, "Stream read failed, reconnecting...")
            }
            Err(error) => tracing::error!("Stream error: {}, reconnecting...", error),
        }
        heartbeat.abort();
        tracing::debug!("Heartbeat cancelled for reconnection");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

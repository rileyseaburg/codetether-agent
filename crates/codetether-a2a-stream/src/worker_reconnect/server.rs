use super::Server;
use crate::{
    breaker::CircuitBreaker,
    reconnect::{apply_lifecycle, log_outcome, make_backoff},
};
use anyhow::Result;
use tokio::sync::mpsc;
/// Reconnect an integrated worker using the shared backoff and breaker policy.
///
/// # Arguments
/// * `host` — Worker services and HTTP server notification/connection state.
/// # Returns
/// Runs until cancelled; holds the transport-probe handle for the loop lifetime.
/// # Errors
/// Registration, fetch and connection failures are logged before retrying.
/// # Examples
/// ```ignore
/// run_worker_server_loop(&integrated_worker).await?;
/// ```
pub async fn run_worker_server_loop<H: Server>(host: &H) -> Result<()> {
    let mut backoff = make_backoff();
    let mut breaker = CircuitBreaker::new(5);
    let _probe = crate::probe::spawn_transport_probe(host.server_url());
    loop {
        let codebases = host.codebases().await;
        let (notify_tx, notify_rx) = mpsc::channel::<String>(32);
        host.set_notification_channel(notify_tx).await;
        host.set_connected(false).await;
        if let Err(error) = host.register().await {
            tracing::warn!(error = %error, "Failed to re-register worker on reconnection");
        }
        if let Err(error) = host.fetch_pending().await {
            tracing::warn!(error = %error, "Reconnect task fetch failed");
        }
        let heartbeat = host.heartbeat();
        let outcome = host.connect(&codebases, Some(notify_rx)).await;
        let connected = log_outcome(&outcome);
        host.set_connected(false).await;
        heartbeat.abort();
        apply_lifecycle(&mut backoff, &mut breaker, connected).await;
    }
}

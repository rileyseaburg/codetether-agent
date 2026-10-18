//! Start or coalesce a worker for a durable session archive.
use super::{ArchiveSource, queue};
use crate::HistorySinkConfig;

/// Start archival work or coalesce a wakeup for an already-running session.
/// # Arguments
/// `id` is the durable session identity; `config` is the selected history sink.
/// `H` loads the same durable consumer snapshots used by the application.
/// # Returns
/// Immediately after scheduling; worker failures are logged and release the key.
/// # Panics
/// Requires a Tokio runtime and unpoisoned queue synchronization.
/// # Examples
/// ```rust,no_run
/// # fn example<H: codetether_history_sink::archive::ArchiveSource>(config: codetether_history_sink::HistorySinkConfig) {
/// codetether_history_sink::archive::schedule::<H>("session-123".into(), config);
/// # }
/// ```
pub fn schedule<H: ArchiveSource>(id: String, config: HistorySinkConfig) {
    if !queue::start(&id) {
        return;
    }
    tokio::spawn(async move {
        loop {
            if let Err(error) = super::drain::drain::<H>(&id, &config).await {
                tracing::warn!(target: "codetether_agent::session::store::archive", session_id = %id, %error, "incremental history upload failed");
                queue::remove(&id);
                break;
            }
            if !queue::again(&id) {
                break;
            }
        }
    });
}

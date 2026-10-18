//! Drain the latest pending generation until its slot settles.
use codetether_proactive_runtime as registry;

/// Spawn the coalescing worker loop for a registered session.
/// # Arguments
/// * `session_id` — Runtime-registry session key.
/// * `host` — Native cursor, projection and sidecar operations.
/// # Panics
/// Panics when called without a running Tokio runtime.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_proactive_worker::Host + 'static>(host: std::sync::Arc<H>) {
/// codetether_proactive_worker::spawn("session".into(), host);
/// # }
/// ```
pub fn spawn<H: crate::Host + 'static>(session_id: String, host: std::sync::Arc<H>) {
    tokio::spawn(async move {
        loop {
            if let Some(snapshot) = registry::take(&session_id) {
                super::incremental::run(snapshot, host.as_ref()).await;
                continue;
            }
            if registry::settle(&session_id) {
                break;
            }
        }
    });
}

use super::Host;
use std::sync::Arc;
use tokio::sync::Notify;
/// Apply review and busy gates, then announce and spawn a prompt.
/// # Arguments
/// `host` supplies native effects; `request` owns the submitted work.
/// # Returns
/// False, so submitting work does not stop the command loop.
/// # Examples
/// ```no_run
/// # async fn submit<Q: Send>(host: &mut impl codetether_session_runtime::dispatch::Host<Q>, request: Q) {
/// assert!(!codetether_session_runtime::dispatch::submit(host, request).await);
/// # }
/// ```
pub async fn submit<Q: Send>(host: &mut impl Host<Q>, request: Q) -> bool {
    if host.held(&request) {
        host.release_prepared(&request);
        host.reject(
            request,
            "Goal paused: select Yes in the answer satisfaction prompt or submit /continue",
        )
        .await;
        return false;
    }
    let notify = Arc::new(Notify::new());
    if !host.attach(&request, Arc::clone(&notify)) {
        host.reject(request, "Session runtime is busy").await;
        return false;
    }
    host.started().await;
    host.spawn(request, notify);
    false
}

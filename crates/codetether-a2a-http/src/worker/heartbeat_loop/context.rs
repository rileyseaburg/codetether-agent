//! Start a connection heartbeat using the resources retained by worker startup.
use codetether_a2a_worker_core::initialization::WorkerContext;
use reqwest::Client;
use tokio::task::JoinHandle;

/// Start heartbeat scheduling for one worker connection attempt.
/// # Arguments
/// `context` supplies the HTTP client and shared worker state; `token` reads
/// authentication from the caller's argument type without changing its storage.
/// # Returns
/// The heartbeat task handle, to be aborted when this connection ends.
/// # Panics
/// Requires an active Tokio runtime, as does [`super::start_heartbeat`].
/// # Examples
/// ```no_run
/// # fn example<B, P>(context: &codetether_a2a_worker_core::initialization::WorkerContext<Option<String>, reqwest::Client, B, P>) {
/// let task = codetether_a2a_http::worker::start_connection_heartbeat(context, Clone::clone);
/// task.abort();
/// # }
/// ```
pub fn start_connection_heartbeat<A, B, P>(
    context: &WorkerContext<A, Client, B, P>,
    token: impl FnOnce(&A) -> Option<String>,
) -> JoinHandle<()> {
    super::start_heartbeat(
        context.task_runtime.client.clone(),
        context.server.clone(),
        token(&context.args),
        context.heartbeat_state.clone(),
        context.processing.clone(),
        context.cognition_heartbeat.clone(),
        context.task_progress.clone(),
    )
}

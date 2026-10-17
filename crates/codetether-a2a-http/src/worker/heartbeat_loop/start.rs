//! Spawn the persistent worker heartbeat task.
use super::*;
/// Start worker heartbeat scheduling and optional active-task lease renewal.
///
/// # Arguments
/// * `client` - HTTP client for heartbeat and cognition requests.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `heartbeat_state` - Shared worker status and sub-agent names.
/// * `processing` - Active task IDs counted on each tick.
/// * `cognition_config` - Optional cognition retrieval settings.
/// * `task_progress` - Active-task progress snapshot.
/// # Returns
/// A background task handle; abort it when the worker stops.
/// # Panics
/// Panics if called outside a Tokio runtime.
/// # Examples
/// ```rust,no_run
/// use codetether_a2a_http::worker::start_heartbeat;
/// let _start = start_heartbeat;
/// ```
pub fn start_heartbeat(
    client: Client,
    server: String,
    token: Option<String>,
    heartbeat_state: HeartbeatState,
    processing: Arc<Mutex<HashSet<String>>>,
    cognition_config: CognitionHeartbeatConfig,
    task_progress: Arc<Mutex<TaskProgressState>>,
) -> JoinHandle<()> {
    tokio::spawn(run::run(
        client,
        server,
        token,
        heartbeat_state,
        processing,
        cognition_config,
        task_progress,
    ))
}

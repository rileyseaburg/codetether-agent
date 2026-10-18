//! Stable registration and stream identity for one bridge.
#[derive(Clone)]
pub(crate) struct Identity {
    pub(crate) server: String,
    pub(crate) token: String,
    pub(crate) worker_id: String,
    pub(crate) worker_name: String,
}
impl Identity {
    pub(crate) fn new(server: String, token: String, worker_name: Option<String>) -> Self {
        // Generate worker ID and name
        let worker_id = codetether_a2a_worker_core::generate_worker_id();
        let worker_name = worker_name.unwrap_or_else(|| format!("tui-{}", std::process::id()));

        tracing::info!(
            worker_id = %worker_id,
            worker_name = %worker_name,
            server = %server,
            "Starting TUI worker bridge"
        );

        Self {
            server,
            token,
            worker_id,
            worker_name,
        }
    }
}

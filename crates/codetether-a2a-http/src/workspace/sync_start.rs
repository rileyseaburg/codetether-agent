//! Background workspace sync loop starter.

use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Start periodic workspace synchronization after the first interval elapses.
///
/// # Arguments
/// * `client` - HTTP client owned by the background task.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `shared_codebases` - Shared local path list to extend.
/// * `configure_entry` - Repository setup callback for each returned entry.
/// # Returns
/// A task handle; abort it to stop synchronization.
/// # Panics
/// Panics if called outside a Tokio runtime.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
/// use codetether_a2a_http::workspace::start_workspace_sync;
/// let handle = start_workspace_sync(reqwest::Client::new(), "http://localhost:8001".into(), None, Arc::new(Mutex::new(Vec::new())), |_| {});
/// handle.abort();
/// # }
/// ```
pub fn start_workspace_sync(
    client: Client,
    server: String,
    token: Option<String>,
    shared_codebases: Arc<Mutex<Vec<String>>>,
    mut configure_entry: impl FnMut(&serde_json::Value) + Send + 'static,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        interval.tick().await;
        loop {
            interval.tick().await;
            if let Err(error) = super::sync_workspaces_from_server(
                &client,
                &server,
                &token,
                &shared_codebases,
                &mut configure_entry,
            )
            .await
            {
                tracing::warn!(target: "codetether_agent::a2a::worker::workspace_sync_start", error = %error, "Workspace sync failed");
            }
        }
    })
}

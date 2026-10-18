//! Resolve configuration and start one bridge runtime.
use crate::{Bridge, Registration, WorkerBridgeCmd, context::Context, identity::Identity};
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_worker_events::IncomingTask;
use std::sync::Arc;
use tokio::sync::mpsc;
/// Start the bridge, returning None when server credentials are unavailable.
/// # Arguments
/// * `server_url` - Explicit server, or load it from application configuration.
/// * `token` - Explicit token, or use saved credentials.
/// * `worker_name` - Explicit worker name, or derive one from the process.
/// * `bus` - Shared agent bus.
/// # Returns
/// Running bridge handles, or None when connection settings are absent.
/// # Panics
/// Panics when called without a Tokio runtime once spawning is required.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_tui_worker::Registration>(bus: std::sync::Arc<codetether_bus_core::AgentBus>) -> anyhow::Result<()> {
/// let _bridge = codetether_tui_worker::spawn::<R>(None, None, None, bus).await?;
/// # Ok(()) }
/// ```
/// # Errors
/// Returns configuration loading errors.
pub async fn spawn<R: Registration>(
    server_url: Option<String>,
    token: Option<String>,
    worker_name: Option<String>,
    bus: Arc<AgentBus>,
) -> Result<Option<Bridge>> {
    let Some((server, token)) = crate::config::resolve(server_url, token).await? else {
        return Ok(None);
    };
    let identity = Identity::new(server, token, worker_name);
    let (cmd_tx, cmd_rx) = mpsc::channel::<WorkerBridgeCmd>(256);
    let (task_tx, task_rx) = mpsc::channel::<IncomingTask>(64);
    let context = Context::new(identity.clone(), bus, cmd_rx, task_tx);
    let handle = tokio::spawn(crate::runtime::run::<R>(context));
    Ok(Some(Bridge {
        worker_id: identity.worker_id,
        worker_name: identity.worker_name,
        cmd_tx,
        task_rx,
        handle,
    }))
}

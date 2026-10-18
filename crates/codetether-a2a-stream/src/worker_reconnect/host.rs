use crate::ingest::StreamDisconnectReason;
use std::future::Future;
use tokio::{sync::mpsc, task::JoinHandle};
/// Native operations needed for each worker connection attempt.
/// # Implementors
/// Application adapters retain HTTP clients, credentials, sessions and buses.
pub trait Worker: Sync {
    /// Snapshot the currently advertised workspaces.
    fn codebases(&self) -> impl Future<Output = Vec<String>> + Send;
    /// Register the current worker identity and capabilities.
    /// # Errors
    /// Returns native registration errors for logging and retry.
    fn register(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Fetch and dispatch pending tasks before connecting.
    /// # Errors
    /// Returns native pending-task errors for logging and retry.
    fn fetch_pending(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Start a heartbeat task that is aborted after the stream disconnects.
    fn heartbeat(&self) -> JoinHandle<()>;
    /// Consume a stream, optionally receiving server-side task notifications.
    /// # Errors
    /// Returns native stream connection errors for logging and retry.
    fn connect(
        &self,
        codebases: &[String],
        notifications: Option<mpsc::Receiver<String>>,
    ) -> impl Future<Output = anyhow::Result<StreamDisconnectReason>> + Send;
}
/// HTTP server integration used by the backoff-driven worker loop.
/// # Implementors
/// Native server adapters bind task notifications and connection status.
pub trait Server: Worker {
    /// Server URL used by the background transport probe.
    fn server_url(&self) -> &str;
    /// Replace the task-notification sender for this connection attempt.
    fn set_notification_channel(
        &self,
        sender: mpsc::Sender<String>,
    ) -> impl Future<Output = ()> + Send;
    /// Publish whether the stream is currently connected.
    fn set_connected(&self, connected: bool) -> impl Future<Output = ()> + Send;
}

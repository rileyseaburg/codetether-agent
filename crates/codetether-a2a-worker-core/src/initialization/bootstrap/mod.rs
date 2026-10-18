//! Initial worker registration, workspace synchronization and task intake.
//!
//! [bootstrap_worker] attempts the startup steps in order and schedules ongoing
//! synchronization. Implement [Host] to retain native transports and task state.
mod run;
pub use run::bootstrap_worker;
use std::future::Future;
/// Native operations used by the shared startup sequence.
/// # Implementors
/// Worker adapters bind registration, workspace sync and pending-task dispatch.
pub trait Host: Sync {
    /// Handle returned by ongoing workspace synchronization.
    type SyncHandle;
    /// Register the current worker.
    /// # Errors
    /// Returns registration failures to be logged without aborting startup.
    fn register(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Synchronize configured workspaces once.
    /// # Errors
    /// Returns synchronization failures to be logged without aborting startup.
    fn sync_workspaces(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Fetch and dispatch tasks pending at startup.
    /// # Errors
    /// Returns pending-task errors to be logged without aborting startup.
    fn fetch_pending(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Schedule ongoing synchronization; its handle is dropped as startup ends.
    fn start_workspace_sync(&self) -> Self::SyncHandle;
}

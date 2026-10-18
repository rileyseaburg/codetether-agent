//! Worker event dispatch and durable resume-cursor updates.
//!
//! [handle_frame] classifies server events, dispatches task notifications, and
//! commits sequenced events after dispatch. Resync events reset the cursor before
//! [Host::reconcile] cold-polls pending tasks. Implement [Host] to bind transport
//! events to the application's task runtime.
mod resync;
mod run;
pub use run::handle_frame;
use std::future::Future;

/// Application operations used by resumable worker events.
///
/// # Implementors
/// Workers supply claim-gated task dispatch and pending-task reconciliation.
pub trait Host: Sync {
    /// Dispatch a parsed task notification before its sequence is committed.
    fn dispatch(&self, task: &serde_json::Value) -> impl Future<Output = ()> + Send;
    /// Cold-poll pending tasks after clearing a stale resume cursor.
    ///
    /// # Errors
    /// Returns the native pending-task polling error, which the caller logs.
    fn reconcile(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
}

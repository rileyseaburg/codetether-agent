//! Pending-task retrieval followed by ordered dispatch.
//!
//! [fetch_pending_tasks] reports every startup/reconnect fetch; [poll_pending_tasks]
//! reports nonempty periodic fetches. Implement [Host] to supply native retrieval
//! and the existing claim-gated task dispatcher.
mod fetch;
mod poll;
pub use fetch::fetch_pending_tasks;
pub use poll::poll_pending_tasks;
use std::future::Future;
/// Native pending-task retrieval and dispatch operations.
/// # Implementors
/// Worker adapters retain HTTP authentication and runtime task ownership.
pub trait Host: Sync {
    /// Retrieve pending tasks for the configured agent.
    /// # Errors
    /// Returns native transport or response-decoding failures.
    fn pending_tasks(&self) -> impl Future<Output = anyhow::Result<Vec<serde_json::Value>>> + Send;
    /// Dispatch one retrieved task through the normal claim path.
    fn dispatch(&self, task: &serde_json::Value) -> impl Future<Output = ()> + Send;
}

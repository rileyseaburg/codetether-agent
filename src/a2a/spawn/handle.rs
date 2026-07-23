//! Lifecycle handle for a background A2A peer endpoint.
//!
//! Dropping the handle stops its tasks, unregisters mDNS through the owned
//! daemon handle, and clears the process-local peer identity.

use crate::a2a::{local_identity, mdns};
use tokio::task::JoinHandle;

/// Owns the tasks and advertised identity of a background A2A endpoint.
///
/// Keep this value alive for as long as the endpoint should remain available.
/// Dropping it immediately stops the endpoint and clears its local identity.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo(handle: codetether_agent::a2a::spawn::A2APeerHandle) {
/// assert!(!handle.agent_name.is_empty());
/// handle.abort();
/// # }
/// ```
pub struct A2APeerHandle {
    /// Exact peer name advertised through A2A discovery.
    pub agent_name: String,
    /// Socket address on which the endpoint listens.
    pub bind_addr: String,
    /// Public URL published in the endpoint's agent card.
    pub public_url: String,
    pub(super) server_task: JoinHandle<()>,
    pub(super) discovery_task: JoinHandle<()>,
    pub(super) mdns_intake_task: Option<JoinHandle<()>>,
    pub(super) lan_tasks: Vec<JoinHandle<()>>,
    pub(super) _mdns_handle: Option<mdns::MdnsHandle>,
}

impl A2APeerHandle {
    /// Abort all background tasks and shut down mDNS immediately.
    pub fn abort(self) {
        drop(self);
    }
}

impl Drop for A2APeerHandle {
    fn drop(&mut self) {
        self.server_task.abort();
        self.discovery_task.abort();
        if let Some(task) = self.mdns_intake_task.as_ref() {
            task.abort();
        }
        self.lan_tasks.iter().for_each(JoinHandle::abort);
        local_identity::deactivate(&self.agent_name);
    }
}

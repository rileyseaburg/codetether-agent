//! Owned peer tasks, advertisement and public identity.
use super::*;

/// Owns the tasks and advertised identity of a background A2A endpoint.
///
/// Keep this value alive for as long as the endpoint should remain available.
/// Dropping it immediately stops the endpoint and clears its local identity.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn demo(handle: codetether_a2a_http::peer_handle::A2APeerHandle) {
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

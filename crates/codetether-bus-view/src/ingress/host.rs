//! Native identity, cursor and presentation bindings for bus ingress.
use codetether_bus_core::{BusEnvelope, BusMessage};
use codetether_worker_events::IncomingTask;
use std::collections::HashSet;
/// Frontend cursor and local agent identities.
/// # Implementors
/// Frontends lend their recorder cursor and peer registration set.
pub trait State {
    /// Last consumed recorder cursor.
    fn cursor(&mut self) -> &mut u64;
    /// Peer identities updated by ready, heartbeat and shutdown messages.
    fn agents(&mut self) -> &mut HashSet<String>;
    /// Check peer registration without changing it.
    fn is_registered(&self, agent: &str) -> bool;
    /// Current worker identity, if present.
    fn worker_id(&self) -> Option<&str>;
    /// Current chat session identity, if present.
    fn session_id(&self) -> Option<&str>;
}
/// Observers, ownership checks and presentation effects for accepted envelopes.
/// # Implementors
/// Frontends preserve their task trackers, queues and chat storage.
pub trait Effects {
    /// Observe peer task activity.
    fn observe_tasks(&mut self, message: &BusMessage);
    /// Observe tool-call activity.
    fn observe_tools(&mut self, message: &BusMessage);
    /// Append the envelope to the protocol log.
    fn ingest_log(&mut self, envelope: &BusEnvelope);
    /// Queue an admitted external task.
    fn enqueue_task(&mut self, task: IncomingTask);
    /// Check whether the sender is a managed child of the current session.
    fn owns_agent(&self, parent: &str, agent: &str) -> bool;
    /// Append a sanitized assistant result to chat.
    fn push_assistant(&mut self, text: String);
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
    /// Scroll chat to the result.
    fn scroll_to_bottom(&mut self);
}
/// Complete frontend bindings for bus ingress.
/// # Implementors
/// Automatically implemented for State + Effects adapters.
pub trait Host: State + Effects {}
impl<T: State + Effects> Host for T {}

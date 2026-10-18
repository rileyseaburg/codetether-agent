//! Frontend state and relay transport bindings for objective approval.
use crate::PendingOkrApproval;
use codetether_okr::OkrRepository;
use std::sync::Arc;
/// Frontend operations used when deciding a pending relay objective.
/// # Implementors
/// Frontends retain their relay event type and bind existing state fields.
pub trait Host {
    /// Receiver returned by relay startup.
    type Receiver;
    /// Pending objective and run, consumed by a decision.
    fn pending(&mut self) -> &mut Option<PendingOkrApproval>;
    /// Optional persistence repository.
    fn repository(&self) -> Option<&Arc<OkrRepository>>;
    /// Start the existing background relay worker.
    fn start_relay(task: String, model: String) -> Self::Receiver;
    /// Update the relay running flag.
    fn set_relay_running(&mut self, running: bool);
    /// Install the worker's event receiver.
    fn set_relay_receiver(&mut self, receiver: Self::Receiver);
    /// Append a system message to the chat transcript.
    fn push_system(&mut self, text: String);
    /// Scroll the chat transcript to the newest content.
    fn scroll_to_bottom(&mut self);
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
}

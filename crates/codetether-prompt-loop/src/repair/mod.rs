//! Transcript repair for provider-truncated tool calls.
//!
//! [record] preserves event/message ordering through a native [Host]. It adds
//! synthetic failures for truncated calls and requests a retry only when no
//! complete calls remain for execution.
mod record;
use codetether_provider_core::Message;
use codetether_session_events::SessionEvent;
pub use record::record;
use tokio::sync::mpsc;
/// Native transcript and stream bindings for truncated-call repair.
/// # Implementors
/// The prompt runner exposes its event sender and session insertion operation.
pub trait Host: Send {
    /// Session snapshot type carried by the existing event stream.
    type Snapshot: Send;
    /// Optional sender receiving synthetic tool failures.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
    /// Append a message through the session's ordinary insertion path.
    fn add_message(&mut self, message: Message);
    /// Clear accumulated answer text before retrying an entirely truncated turn.
    fn clear_output(&mut self);
}

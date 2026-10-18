//! Deliver RLM progress and completion through a typed durable session bus.
//!
//! [`DurableSessionBus`] implements [`crate::Bus`] for any thread-safe snapshot
//! type. Events use the same durability and broadcast path as other session
//! activity, with no additional buffering or payload conversion.
use codetether_session_bus::{DurableSessionBus, SessionEvent};

impl<S: Clone + Send + Sync + 'static> crate::Bus for DurableSessionBus<S> {
    fn emit_progress(&self, event: codetether_rlm::RlmProgressEvent) {
        self.emit(SessionEvent::RlmProgress(event));
    }
    fn emit_completion(&self, event: codetether_rlm::RlmCompletion) {
        self.emit(SessionEvent::RlmComplete(event));
    }
}

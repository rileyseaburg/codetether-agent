//! Bind shared RLM notifications to the native session bus.
use crate::session::{SessionBus, SessionEvent};
impl codetether_rlm_routing::Bus for SessionBus {
    fn emit_progress(&self, event: codetether_rlm::RlmProgressEvent) {
        self.emit(SessionEvent::RlmProgress(event));
    }
    fn emit_completion(&self, event: codetether_rlm::RlmCompletion) {
        self.emit(SessionEvent::RlmComplete(event));
    }
}

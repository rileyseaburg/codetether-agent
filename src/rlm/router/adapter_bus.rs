//! Event-bus adapter: `SessionBus` → `RlmEventBus`.

use std::sync::Arc;
use codetether_rlm::traits::RlmEventBus;
use crate::session::SessionEvent;

/// Wraps `SessionBus` as `RlmEventBus`.
pub(super) struct BusWrap(pub(crate) crate::session::SessionBus);

impl RlmEventBus for BusWrap {
    fn emit_progress(&self, event: codetether_rlm::RlmProgressEvent) {
        // RlmProgressEvent is the same type as session::RlmProgressEvent (re-export)
        self.0.emit(SessionEvent::RlmProgress(event));
    }

    fn emit_completion(&self, event: codetether_rlm::RlmCompletion) {
        // RlmCompletion is the same type as session::RlmCompletion (re-export)
        self.0.emit(SessionEvent::RlmComplete(event));
    }
}

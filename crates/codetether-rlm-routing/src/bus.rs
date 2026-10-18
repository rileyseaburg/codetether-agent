//! Generic RLM event delivery through an application bus.
use codetether_rlm::{RlmCompletion, RlmProgressEvent, traits::RlmEventBus};
/// Deliver RLM events using an application's existing transport semantics.
/// # Implementors
/// Native buses map these values into their corresponding session events.
pub trait Bus: Send + Sync + 'static {
    /// Emit one progress notification.
    fn emit_progress(&self, event: RlmProgressEvent);
    /// Emit one completion notification.
    fn emit_completion(&self, event: RlmCompletion);
}
pub(crate) struct BusWrap<B: Bus>(pub(crate) B);
impl<B: Bus> RlmEventBus for BusWrap<B> {
    fn emit_progress(&self, event: RlmProgressEvent) {
        self.0.emit_progress(event);
    }
    fn emit_completion(&self, event: RlmCompletion) {
        self.0.emit_completion(event);
    }
}

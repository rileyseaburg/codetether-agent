//! RlmEventBus host integration contract.

/// Trait for emitting RLM events (progress + completion).
///
/// # Implementors
/// The embedding application implements this interface to supply model, tool, or event services.
pub trait RlmEventBus: Send + Sync {
    /// Emit a progress tick.
    fn emit_progress(&self, event: crate::RlmProgressEvent);
    /// Emit the terminal completion record.
    fn emit_completion(&self, event: crate::RlmCompletion);
}

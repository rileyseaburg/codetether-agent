//! Frontend bindings for context telemetry presentation.
use crate::ContextHealthState;
/// Mutable health state and presentation fields used by event handlers.
/// # Implementors
/// Frontends expose their existing health state and token/status fields.
pub trait View {
    /// Borrow recent context and RLM health history.
    fn health(&mut self) -> &mut ContextHealthState;
    /// Update the token usage estimate.
    fn set_used(&mut self, tokens: usize);
    /// Update the token budget.
    fn set_budget(&mut self, tokens: usize);
    /// Replace the frontend status message.
    fn set_status(&mut self, status: String);
    /// Append a context-compaction error to the transcript.
    fn push_error(&mut self, text: String);
}

//! Completion metric and usage-badge presentation bindings.
use codetether_chat_model::ChatMessage;
/// Frontend fields updated by a completion usage report.
/// # Implementors
/// Frontends expose completion metrics, messages and status text.
pub trait UsageView {
    /// Record the model used for the completion.
    fn set_model(&mut self, model: String);
    /// Record elapsed completion milliseconds.
    fn set_latency(&mut self, duration_ms: u64);
    /// Record input token usage.
    fn set_prompt_tokens(&mut self, tokens: usize);
    /// Record output token usage.
    fn set_output_tokens(&mut self, tokens: usize);
    /// Borrow chronological chat messages.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
}

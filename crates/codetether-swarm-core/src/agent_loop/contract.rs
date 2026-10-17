use super::*;
use async_trait::async_trait;
/// Host operations used by the shared provider-turn driver.
///
/// # Implementors
/// Application state supplies steering, provider requests, normalization,
/// event publication and tool execution while this crate controls turn order.
#[async_trait]
pub trait AgentRuntime: Send {
    /// Wait for readiness, returning cancellation as an error.
    ///
    /// # Errors
    /// Returns the host cancellation or readiness failure.
    async fn ready(&mut self) -> Result<()>;
    /// Apply pending user input and report whether input was added.
    fn apply_input(&mut self) -> bool;
    /// Check configured step and time limits.
    fn limit_reached(&self) -> Option<AgentLoopExit>;
    /// Increment the turn counter and enforce the history budget.
    fn advance(&mut self);
    /// Return the current step count used in diagnostics.
    fn steps(&self) -> usize;
    /// Make a deadline-bound provider request.
    ///
    /// # Errors
    /// Returns a provider execution error.
    async fn request(&mut self) -> Result<RequestOutcome>;
    /// Normalize provider output before parsing text and tool calls.
    fn normalize(&self, response: CompletionResponse) -> CompletionResponse;
    /// Publish thinking text using the host's event policy.
    fn publish_thinking(&self, text: &str);
    /// Accumulate and publish assistant text.
    fn record_output(&mut self, text: &str);
    /// Append the normalized assistant message to conversation history.
    fn append_message(&mut self, message: Message);
    /// Execute tools and return whether a tool deadline elapsed.
    ///
    /// # Errors
    /// Returns host readiness or tool execution errors.
    async fn execute_tools(&mut self, calls: Vec<ToolCall>) -> Result<bool>;
}

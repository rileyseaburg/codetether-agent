use codetether_provider_core::{CompletionResponse, ToolDefinition};
/// Borrowed provider identity and available tool definitions.
/// Used during prose-call recovery without owning the provider registry.
/// # Examples
/// ```
/// let view = codetether_prompt_loop::driver::ProviderView { provider: "openai", model: "model", tools: &[] };
/// assert!(view.tools.is_empty());
/// ```
pub struct ProviderView<'a> {
    /// Selected provider registry name.
    pub provider: &'a str,
    /// Selected provider model identifier.
    pub model: &'a str,
    /// Available tool definitions.
    pub tools: &'a [ToolDefinition],
}
/// Native response-guard bindings.
/// # Implementors
/// The session runner delegates to shared retry policies and applies their effects.
pub trait Guards {
    /// Borrow model identity and tools for call recovery.
    fn provider_view(&self) -> ProviderView<'_>;
    /// Retry a build response that has not executed required tools.
    /// # Errors
    /// Propagates the existing build guard error.
    fn tool_first(&mut self, text: &str, calls: bool) -> anyhow::Result<bool>;
    /// Retry missing promised calls or report exhausted build retries.
    /// # Errors
    /// Propagates the existing exhaustion error.
    fn native_call(
        &mut self,
        response: &CompletionResponse,
        text: &str,
        calls: bool,
    ) -> anyhow::Result<bool>;
    /// Apply thinking-only continuation guidance if needed.
    fn thinking_only(&mut self, response: &CompletionResponse) -> bool;
}

//! Optional host response-rewriting interface.
use codetether_provider_core::{CompletionResponse, ToolDefinition};
/// Reformat a provider response using host-owned tool-routing services.
///
/// # Implementors
/// The application implements this trait for its local FunctionGemma router.
/// Implementations return the response directly, preserving host fallback policy.
#[async_trait::async_trait]
pub trait ResponseRewriter: Send + Sync {
    /// Reformat a response with the caller's native-tool support flag.
    async fn maybe_reformat(
        &self,
        response: CompletionResponse,
        tools: &[ToolDefinition],
        model_supports_tools: bool,
    ) -> CompletionResponse;
}

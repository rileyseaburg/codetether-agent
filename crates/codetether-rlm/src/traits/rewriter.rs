//! ToolCallRewriter host integration contract.

use super::{LlmResponse, ToolDefinition};
use anyhow::Result;
use async_trait::async_trait;

/// Trait for FunctionGemma-powered tool call reformatting.
///
/// # Implementors
/// The embedding application implements this interface to supply model, tool, or event services.
#[async_trait]
pub trait ToolCallRewriter: Send + Sync {
    /// Extract structured tool calls from a text-only response.
    async fn maybe_reformat(
        &self,
        response: LlmResponse,
        tools: &[ToolDefinition],
        strict: bool,
    ) -> Result<LlmResponse>;
}

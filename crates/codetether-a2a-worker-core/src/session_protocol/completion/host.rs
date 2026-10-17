//! Host completion operations retaining application context preparation.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{CompletionResponse, ToolDefinition};
/// Initial and compact-schema retry operations for a worker completion.
///
/// # Implementors
/// Application hosts own provider/session context and request options. Retry
/// consumes the host, allowing provider ownership to pass into the final call.
#[async_trait]
pub trait CompletionHost: Send {
    /// Attempt completion with the original tool schemas.
    /// # Errors
    /// Returns the provider/context preparation error.
    async fn complete_initial(&mut self, tools: &[ToolDefinition]) -> Result<CompletionResponse>;
    /// Retry once with compacted schemas, consuming the completion context.
    /// # Errors
    /// Returns the final provider/context preparation error.
    async fn complete_retry(self, tools: &[ToolDefinition]) -> Result<CompletionResponse>;
}

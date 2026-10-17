//! LlmProvider host integration contract.

use super::{LlmMessage, LlmResponse, ToolDefinition};
use anyhow::Result;
use async_trait::async_trait;

/// Narrow provider trait for LLM completions.
///
/// # Implementors
/// The embedding application implements this interface to supply model, tool, or event services.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a single completion.
    async fn complete(
        &self,
        messages: Vec<LlmMessage>,
        tools: Vec<ToolDefinition>,
        model: &str,
        temperature: Option<f32>,
    ) -> Result<LlmResponse>;
}

//! Host execution interface for selected search backends.
use crate::types::BackendChoice;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
/// Executes an already selected backend in the caller's environment.
///
/// # Implementors
/// Application adapters implement this trait using their tool registry or
/// concrete tools. Shared search orchestration does not construct host tools.
#[async_trait]
pub trait BackendExecutor: Send + Sync {
    /// Execute a choice using the original query for missing arguments.
    ///
    /// # Errors
    /// Returns an error when the host cannot execute the selected backend.
    async fn run_choice(&self, choice: &BackendChoice, query: &str) -> Result<ToolResult>;
}

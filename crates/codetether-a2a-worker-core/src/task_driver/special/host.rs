//! Runtime policy and special-task execution supplied by the application.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde_json::{Map, Value};
/// Application operations for clone and forage task dispatch.
///
/// # Implementors
/// Hosts preserve runtime approval policy and delegate domain execution.
#[async_trait]
pub trait SpecialTaskHost: Sync {
    /// Return a blocking result when runtime policy rejects or holds an action.
    async fn evaluate(&self, tool: &str, args: &Value) -> Option<ToolResult>;
    /// Clone or refresh the task's workspace and return its result text.
    /// # Errors
    /// Returns clone-task execution errors for failed-result mapping.
    async fn clone_repo(&self, task: &Value, metadata: &Map<String, Value>) -> Result<String>;
    /// Execute a forage task and return its result text.
    /// # Errors
    /// Returns forage execution errors for failed-result mapping.
    async fn forage(
        &self,
        title: &str,
        prompt: &str,
        metadata: &Map<String, Value>,
        selected_model: Option<String>,
    ) -> Result<String>;
}

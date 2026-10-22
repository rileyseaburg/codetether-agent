//! Worker tool calls using native execution and application input enrichment.
//!
//! [`execute_tool_call`] retains the shared approval and result sequence.
//! [`InputContext`] supplies session-aware input enrichment and context refresh; [`Runner`]
//! connects that operation to the native response driver.
use codetether_a2a_worker_core::{
    execution_policy::AutoApprove,
    session_protocol::{ToolCall, transcript},
};
use codetether_application_registry::{Application, ToolRegistry};
use std::{path::Path, sync::Arc};
mod host;
mod response;
mod run;
pub use response::Runner;
pub use run::execute_tool_call;

/// Session-aware input enrichment needed by native worker tool calls.
/// # Implementors
/// Implement on the application session using its existing model/workspace
/// enrichment function. Transcript insertion and persistence remain inherited.
#[async_trait::async_trait]
pub trait InputContext: transcript::Host {
    /// Evidence-backed repeated-edit rejection shared with other session runners.
    fn repeated_edit(&self, tool: &str, input: &serde_json::Value) -> Option<String>;
    /// Record actual status before the worker's raw message encoder discards it.
    fn record_outcome(&mut self, id: &str, result: &codetether_tool_core::ToolResult);
    /// Reload externally edited context after the current transcript is persisted.
    /// # Errors
    /// Propagates source loading or decoding failures.
    async fn refresh_context(&mut self) -> anyhow::Result<()>;
    /// Enrich approved tool input for the active workspace and selected model.
    fn prepare_worker_input(
        &self,
        input: &serde_json::Value,
        workspace: &Path,
        model: &str,
    ) -> serde_json::Value;
}

//! Worker tool calls using native execution and application input enrichment.
//!
//! [`execute_tool_call`] retains the shared approval and result sequence.
//! [`InputContext`] supplies only session-aware input enrichment; [`Runner`]
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
pub trait InputContext: transcript::Host {
    /// Enrich approved tool input for the active workspace and selected model.
    fn prepare_worker_input(
        &self,
        input: &serde_json::Value,
        workspace: &Path,
        model: &str,
    ) -> serde_json::Value;
}

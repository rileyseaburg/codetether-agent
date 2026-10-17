//! Application-specific session path and independent-verifier bindings.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use std::path::PathBuf;
/// Application services required by the legacy session-task tool.
///
/// # Implementors
/// Resolve the canonical session journal and forward completion requests to
/// the existing independent goal-verification path.
#[async_trait]
pub trait Host: Send + Sync {
    /// Resolve the session journal path without opening or mutating it.
    /// # Errors
    /// Returns invalid-session or storage-location errors.
    fn log_path(session_id: &str) -> Result<PathBuf>;
    /// Verify a legacy request to complete the session goal.
    /// # Errors
    /// Returns errors from the independent goal update/verifier.
    async fn complete_goal(
        evidence: String,
        session_id: Option<String>,
        current_model: Option<String>,
        workspace: Option<String>,
    ) -> Result<ToolResult>;
}

//! Application version and workspace authorization for RustyRoad execution.
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::Path;
/// Provides application identity and its authoritative workspace policy.
///
/// # Implementors
/// Forward authorization to the existing workspace-aware policy evaluator.
#[async_trait]
pub trait Host: Send + Sync {
    /// Application package version used in the MCP handshake.
    const CLIENT_VERSION: &'static str;
    /// Return a blocking result before a backend process is created.
    async fn evaluate_tool_invocation_for_workspace(
        tool: &str,
        input: &Value,
        cwd: &Path,
    ) -> Option<ToolResult>;
}

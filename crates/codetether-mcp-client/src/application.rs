//! Identity and authorization supplied by the embedding application.
use anyhow::Result;
use serde_json::Value;
use std::future::Future;

/// Policy callbacks shared by all clients and registries for an application.
/// # Implementors
/// Applications supply their package version and existing process/tool policy.
pub trait Application: Send + Sync + 'static {
    /// Version advertised in the MCP initialization request.
    fn version() -> &'static str;
    /// Authorize a subprocess before any process or connection is created.
    /// # Arguments
    /// Receives the original command, arguments and optional approval identifier.
    /// # Returns
    /// Completes when execution is authorized.
    /// # Errors
    /// Returns the application's denial or authorization error.
    fn authorize(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> impl Future<Output = Result<()>> + Send;
    /// Evaluate original tool arguments before approval metadata is stripped.
    /// # Arguments
    /// Receives the requested tool name and unmodified arguments.
    /// # Returns
    /// Denial text, or `None` when the remote invocation may proceed.
    fn blocked(name: &str, arguments: &Value) -> impl Future<Output = Option<String>> + Send;
}

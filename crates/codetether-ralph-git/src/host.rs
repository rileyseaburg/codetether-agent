//! Native invocation-policy evaluation for Git operations.
use codetether_tool_core::ToolResult;
/// Evaluate the same bash invocation policy used by native Ralph commands.
/// # Implementors
/// Return the existing blocked tool result, or None when policy permits execution.
#[async_trait::async_trait]
pub trait Host: Send + Sync {
    /// Evaluate the command/cwd JSON without executing the command.
    /// # Errors
    /// Propagates policy evaluator errors if the native evaluator reports them.
    async fn check(args: serde_json::Value) -> anyhow::Result<Option<ToolResult>>;
}

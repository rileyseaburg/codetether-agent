//! Application policy and score-plugin tool execution.
#[cfg(feature = "tetherscript")]
use codetether_tool_core::ToolResult;
#[cfg(feature = "tetherscript")]
use std::future::Future;
/// Application operations required when TetherScript scoring is enabled.
/// # Implementors
/// Applications forward to their existing authorization and plugin tool.
/// Without the scripting feature, no operations are required.
pub trait Host {
    /// Evaluate the original plugin invocation before execution.
    #[cfg(feature = "tetherscript")]
    fn evaluate_tool_invocation(
        name: &str,
        args: &serde_json::Value,
    ) -> impl Future<Output = Option<ToolResult>> + Send;
    /// Execute the approved invocation with the application plugin tool.
    #[cfg(feature = "tetherscript")]
    fn execute(args: serde_json::Value) -> impl Future<Output = anyhow::Result<ToolResult>> + Send;
}

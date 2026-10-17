//! Async executable-tool interface independent of concrete registries.
use crate::ToolResult;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
/// A tool that can be executed by an agent
/// An executable capability that can be shared across async callers.
///
/// # Examples
/// ```
/// use codetether_tool_core::{Tool, ToolResult};
/// use serde_json::{Value, json};
/// struct Echo;
/// #[async_trait::async_trait]
/// impl Tool for Echo {
///     fn id(&self) -> &str { "echo" }
///     fn name(&self) -> &str { "Echo" }
///     fn description(&self) -> &str { "Return input" }
///     fn parameters(&self) -> Value { json!({"type":"object"}) }
///     async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
///         Ok(ToolResult::success(args.to_string()))
///     }
/// }
/// assert_eq!(Echo.id(), "echo");
/// ```
#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool identifier
    fn id(&self) -> &str;

    /// Human-readable name
    fn name(&self) -> &str;

    /// Description for the LLM
    fn description(&self) -> &str;

    /// JSON Schema for parameters
    fn parameters(&self) -> Value;

    /// Execute the tool with given arguments.
    ///
    /// # Arguments
    /// * `args` - JSON arguments described by this tool's parameter schema.
    ///
    /// # Returns
    /// The tool's text, outcome, and optional structured metadata.
    ///
    /// # Errors
    /// Returns an implementation-defined error if execution cannot produce a result.
    async fn execute(&self, args: Value) -> Result<ToolResult>;
}

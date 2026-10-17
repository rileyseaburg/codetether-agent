//! Shared list collaboration tool.
mod args;
use crate::MessagingHost;
use anyhow::Result;
use args::Args;
pub use args::Args as ListArgs;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;

/// Stateless collaboration tool backed by the existing application messaging runtime.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::MessagingHost>()->codetether_collaboration_tools::ListAgentsTool<H>{
///     codetether_collaboration_tools::ListAgentsTool::new()
/// }
/// ```
pub struct ListAgentsTool<H: MessagingHost>(PhantomData<fn() -> H>);
impl<H: MessagingHost> ListAgentsTool<H> {
    /// Construct the tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: MessagingHost> Tool for ListAgentsTool<H> {
    fn id(&self) -> &str {
        "list_agents"
    }
    fn name(&self) -> &str {
        "List Agents"
    }
    fn description(&self) -> &str {
        "List live agents in the current root thread tree."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "path_prefix":{"type":"string",
                "description":"Task-path prefix without a trailing slash"}
        }})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        let current = args
            .context
            .session_id
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("session id not available"))?;
        let agents = H::list(current, args.path_prefix.as_deref())?;
        Ok(ToolResult::success(json!({"agents":agents}).to_string()))
    }
}

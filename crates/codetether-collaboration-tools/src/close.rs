//! First-class close agent lifecycle operation.
mod args;
use crate::{Host, legacy};
use anyhow::Result;
use args::Args;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;
/// Close a child through the existing runtime.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::Host>()->codetether_collaboration_tools::CloseAgentTool<H> {
///     codetether_collaboration_tools::CloseAgentTool::new()
/// }
/// ```
pub struct CloseAgentTool<H: Host>(PhantomData<fn() -> H>);
impl<H: Host> CloseAgentTool<H> {
    /// Construct the lifecycle tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: Host> Tool for CloseAgentTool<H> {
    fn id(&self) -> &str {
        "close_agent"
    }
    fn name(&self) -> &str {
        "Close Agent"
    }
    fn description(&self) -> &str {
        "Close a sub-agent subtree, preserving durable transcripts and queued work for resume."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"target":{"type":"string"}},
            "required":["target"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        legacy::execute::<H>(
            &args.context,
            json!({"action":"close", "name":args.target})
                .as_object()
                .cloned()
                .unwrap_or_default(),
        )
        .await
    }
}

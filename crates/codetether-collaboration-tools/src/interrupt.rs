//! First-class interrupt agent lifecycle operation.
mod args;
use crate::{Host, legacy};
use anyhow::Result;
use args::Args;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;
/// Interrupt a child through the existing runtime.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::Host>()->codetether_collaboration_tools::InterruptAgentTool<H> {
///     codetether_collaboration_tools::InterruptAgentTool::new()
/// }
/// ```
pub struct InterruptAgentTool<H: Host>(PhantomData<fn() -> H>);
impl<H: Host> InterruptAgentTool<H> {
    /// Construct the lifecycle tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: Host> Tool for InterruptAgentTool<H> {
    fn id(&self) -> &str {
        "interrupt_agent"
    }
    fn name(&self) -> &str {
        "Interrupt Agent"
    }
    fn description(&self) -> &str {
        "Interrupt an active sub-agent turn without deleting its persisted session."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"target":{"type":"string"}},
            "required":["target"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        legacy::execute::<H>(
            &args.context,
            json!({
                "action":"interrupt", "name":args.target
            })
            .as_object()
            .cloned()
            .unwrap_or_default(),
        )
        .await
    }
}

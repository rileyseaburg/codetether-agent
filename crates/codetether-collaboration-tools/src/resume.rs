//! First-class resume agent lifecycle operation.
mod args;
use crate::{Host, legacy};
use anyhow::Result;
use args::Args;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;
/// Resume a child through the existing runtime.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::Host>()->codetether_collaboration_tools::ResumeAgentTool<H> {
///     codetether_collaboration_tools::ResumeAgentTool::new()
/// }
/// ```
pub struct ResumeAgentTool<H: Host>(PhantomData<fn() -> H>);
impl<H: Host> ResumeAgentTool<H> {
    /// Construct the lifecycle tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: Host> Tool for ResumeAgentTool<H> {
    fn id(&self) -> &str {
        "resume_agent"
    }
    fn name(&self) -> &str {
        "Resume Agent"
    }
    fn description(&self) -> &str {
        "Resume a closed durable child thread by its session ID."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"id":{"type":"string"}},
            "required":["id"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        legacy::execute::<H>(
            &args.context,
            json!({"action":"resume", "name":args.id})
                .as_object()
                .cloned()
                .unwrap_or_default(),
        )
        .await
    }
}

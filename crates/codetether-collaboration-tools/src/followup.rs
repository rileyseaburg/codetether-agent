//! Shared followup collaboration tool.
mod args;
mod execute;
use crate::MessagingHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;
mod trigger;
/// Stateless collaboration tool backed by the existing application messaging runtime.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::MessagingHost>()->codetether_collaboration_tools::FollowupTaskTool<H>{
///     codetether_collaboration_tools::FollowupTaskTool::new()
/// }
/// ```
pub struct FollowupTaskTool<H: MessagingHost>(PhantomData<fn() -> H>);
impl<H: MessagingHost> FollowupTaskTool<H> {
    /// Construct the tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: MessagingHost> Tool for FollowupTaskTool<H> {
    fn id(&self) -> &str {
        "followup_task"
    }
    fn name(&self) -> &str {
        "Follow Up Task"
    }
    fn description(&self) -> &str {
        "Steer a running child promptly, or trigger a background turn when it is idle."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "target":{"type":"string"}, "message":{"type":"string"}
        },"required":["target","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        execute::run::<H>(input).await
    }
}

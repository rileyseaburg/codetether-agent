//! Structured child input submission through the existing messaging host.
mod action;
mod args;
mod dispatch;
use crate::MessagingHost;
use anyhow::Result;
pub use args::SendInputArgs;
use async_trait::async_trait;
use codetether_collaboration_input as input;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::marker::PhantomData;
/// Submit text and attachments, optionally interrupting the active child first.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::MessagingHost>()->codetether_collaboration_tools::SendInputTool<H>{
///     codetether_collaboration_tools::SendInputTool::new()
/// }
/// ```
pub struct SendInputTool<H: MessagingHost>(PhantomData<fn() -> H>);
impl<H: MessagingHost> SendInputTool<H> {
    /// Construct the structured input tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: MessagingHost> Tool for SendInputTool<H> {
    fn id(&self) -> &str {
        "send_input"
    }
    fn name(&self) -> &str {
        "Send Input"
    }
    fn description(&self) -> &str {
        "Queue structured input on a child thread and return its durable submission ID."
    }
    fn parameters(&self) -> Value {
        input::parameters()
    }
    async fn execute(&self, value: Value) -> Result<ToolResult> {
        let args: args::SendInputArgs = serde_json::from_value(value)?;
        let prepared =
            input::prepare_in_workspace(args.message, args.items, args.context.workspace()).await?;
        dispatch::execute::<H>(args.context, args.target, prepared, args.interrupt).await
    }
}

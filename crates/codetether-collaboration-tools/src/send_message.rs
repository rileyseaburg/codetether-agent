//! Shared send message collaboration tool.
mod args;
use crate::MessagingHost;
use anyhow::Result;
use anyhow::bail;
use args::Args;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;
/// Stateless collaboration tool backed by the existing application messaging runtime.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::MessagingHost>()->codetether_collaboration_tools::SendMessageTool<H>{
///     codetether_collaboration_tools::SendMessageTool::new()
/// }
/// ```
pub struct SendMessageTool<H: MessagingHost>(PhantomData<fn() -> H>);
impl<H: MessagingHost> SendMessageTool<H> {
    /// Construct the tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: MessagingHost> Tool for SendMessageTool<H> {
    fn id(&self) -> &str {
        "send_message"
    }
    fn name(&self) -> &str {
        "Send Message"
    }
    fn description(&self) -> &str {
        "Deliver context to a child promptly without starting an idle child turn."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "target":{"type":"string"}, "message":{"type":"string"}
        },"required":["target","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        if args.message.trim().is_empty() {
            bail!("Empty message can't be sent to an agent");
        }
        if let Some(result) = crate::ensure::ready::<H>(&args.context, &args.target).await? {
            return Ok(result);
        }
        H::queue_only(
            &args.target,
            args.context.session_id.as_deref(),
            args.message,
        )
        .await
    }
}

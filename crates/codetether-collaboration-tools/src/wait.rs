//! First-class MultiAgentV2 mailbox-activity wait tool.

use crate::WaitHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
use std::marker::PhantomData;

#[path = "wait/args.rs"]
mod args;
#[path = "wait/run.rs"]
pub(crate) mod run;
use args::WaitArgs as Args;
pub use args::WaitArgs;

/// Wait for parent mailbox activity or legacy child status changes.
///
/// # Examples
/// ```
/// fn construct<H:codetether_collaboration_tools::WaitHost>()->codetether_collaboration_tools::WaitAgentTool<H>{
///     codetether_collaboration_tools::WaitAgentTool::new()
/// }
/// ```
pub struct WaitAgentTool<H: WaitHost>(PhantomData<fn() -> H>);
impl<H: WaitHost> WaitAgentTool<H> {
    /// Construct the wait tool.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[async_trait]
impl<H: WaitHost> Tool for WaitAgentTool<H> {
    fn id(&self) -> &str {
        "wait_agent"
    }
    fn name(&self) -> &str {
        "Wait Agent"
    }
    fn description(&self) -> &str {
        "Wait for a mailbox update from any live agent or steered user input."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "timeout_ms":{"type":"integer","minimum":10000,"maximum":3600000,
                "description":"Defaults to 30000 ms; min 10000 ms; max 3600000 ms"}
        }})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        run::execute::<H>(serde_json::from_value(input)?).await
    }
}

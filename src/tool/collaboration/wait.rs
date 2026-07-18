//! First-class MultiAgentV2 mailbox-activity wait tool.

use super::context::RuntimeContext;
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

#[path = "wait/args.rs"]
mod args;
#[path = "wait/run.rs"]
mod run;
use args::Args;

pub(super) struct WaitAgentTool;

#[async_trait]
impl Tool for WaitAgentTool {
    fn id(&self) -> &str { "wait_agent" }
    fn name(&self) -> &str { "Wait Agent" }
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
        run::execute(serde_json::from_value(input)?).await
    }
}

#[cfg(test)]
#[path = "wait_tests.rs"]
mod tests;

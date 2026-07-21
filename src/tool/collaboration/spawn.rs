//! First-class `spawn_agent` tool.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

#[path = "spawn/args.rs"]
mod args;
#[path = "spawn/dispatch.rs"]
mod dispatch;
#[path = "spawn_name.rs"]
mod name;
#[path = "spawn/result.rs"]
mod result;

pub(super) struct SpawnAgentTool;

#[async_trait]
impl Tool for SpawnAgentTool {
    fn id(&self) -> &str {
        "spawn_agent"
    }
    fn name(&self) -> &str {
        "Spawn Agent"
    }
    fn description(&self) -> &str {
        "Spawn a named background sub-agent for one concrete, bounded task. The child inherits the workspace and parent context policy."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "task_name":{"type":"string","pattern":"^[a-z0-9_]+$"},
            "message":{"type":"string"},
            "fork_turns":{"type":"string","description":"Compatibility: none, all, or a count"}
        },"required":["task_name","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        dispatch::execute(serde_json::from_value(input)?).await
    }
}

#[cfg(test)]
#[path = "spawn_tests.rs"]
mod tests;

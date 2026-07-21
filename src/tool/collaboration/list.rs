//! First-class MultiAgentV2 root-tree listing tool.

use super::context::RuntimeContext;
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct ListAgentsTool;

#[derive(Deserialize)]
struct Args {
    path_prefix: Option<String>,
    #[serde(flatten)]
    context: RuntimeContext,
}

#[async_trait]
impl Tool for ListAgentsTool {
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
        let agents = crate::tool::agent::collaboration_runtime::agent_tree::list(
            current,
            args.path_prefix.as_deref(),
        )?;
        Ok(ToolResult::success(json!({"agents":agents}).to_string()))
    }
}

#[cfg(test)]
#[path = "list_tests.rs"]
mod tests;

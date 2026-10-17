//! Application agent-engine execution and canonical path lookup.
use crate::tool::{Tool, ToolResult, agent::AgentTool};
use anyhow::Result;
use serde_json::Value;
/// Existing agent engine used by the shared lifecycle tools.
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_collaboration_tools::Host for ApplicationHost {
    async fn execute(payload: Value) -> Result<ToolResult> {
        AgentTool::new().execute(payload).await
    }
    fn canonical(owner: &str, agent_id: &str) -> Result<Option<String>> {
        crate::tool::agent::collaboration_runtime::agent_tree::canonical(owner, agent_id)
    }
}

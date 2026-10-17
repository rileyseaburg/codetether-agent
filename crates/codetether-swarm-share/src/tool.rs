//! tool for shared swarm results.
use super::*;
#[async_trait]
impl Tool for SwarmShareTool {
    fn id(&self) -> &str {
        "swarm_share"
    }

    fn name(&self) -> &str {
        "Swarm Share"
    }

    fn description(&self) -> &str {
        "Share results with other sub-agents in the swarm. Actions: publish (share a result), \
         get (retrieve a result by key), query_tags (find results by tags), \
         query_prefix (find results by key prefix), list (show all shared results)."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        execute::execute(self, params).await
    }
}

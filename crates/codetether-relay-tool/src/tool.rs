//! tool for relay coordination.
use super::*;
#[async_trait]
impl Tool for RelayAutoChatTool {
    fn id(&self) -> &str {
        "relay_autochat"
    }

    fn name(&self) -> &str {
        "Relay AutoChat"
    }

    fn description(&self) -> &str {
        "Autonomous relay communication between agents for task delegation and result aggregation. \
         Actions: delegate (send task to target agent), handoff (pass context between agents), \
         status (check relay status), list_agents (show available agents in relay), \
         init (initialize a new relay with task), complete (finish relay and aggregate results)."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.dispatch(params).await
    }
}

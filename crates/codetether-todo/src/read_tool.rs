//! read tool for session todo tools.
use super::*;
#[async_trait]
impl Tool for TodoReadTool {
    fn id(&self) -> &str {
        "todoread"
    }

    fn name(&self) -> &str {
        "Todo Read"
    }

    fn description(&self) -> &str {
        "Read todo items. Filter by status (pending/in_progress/done/blocked) or priority (low/medium/high/critical)."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "status": {"type": "string", "enum": ["pending", "in_progress", "inprogress", "done", "blocked"]},
                "priority": {"type": "string", "enum": ["low", "medium", "high", "critical"]}
            }
        })
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        read::execute(self, params)
    }
}

//! write tool for session todo tools.
use super::*;
#[async_trait]
impl Tool for TodoWriteTool {
    fn id(&self) -> &str {
        "todowrite"
    }

    fn name(&self) -> &str {
        "Todo Write"
    }

    fn description(&self) -> &str {
        "Manage todo items: add, update, delete, or clear todos."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {"type": "string", "enum": ["add", "update", "delete", "clear"], "description": "Action to perform"},
                "id": {"type": "string", "description": "Todo ID (for update/delete)"},
                "content": {"type": "string", "description": "Todo content (for add/update)"},
                "status": {"type": "string", "enum": ["pending", "in_progress", "inprogress", "done", "blocked"]},
                "priority": {"type": "string", "enum": ["low", "medium", "high", "critical"]}
            },
            "required": ["action"]
        })
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        write::execute(self, params)
    }
}

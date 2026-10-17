//! exit tool for the planning tools.
use super::*;
#[async_trait]
impl Tool for PlanExitTool {
    fn id(&self) -> &str {
        "plan_exit"
    }

    fn name(&self) -> &str {
        "Exit Plan Mode"
    }

    fn description(&self) -> &str {
        "Exit planning mode. Optionally mark a step as complete or provide a summary."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "summary": {"type": "string", "description": "Summary of what was accomplished"},
                "step_complete": {"type": "integer", "description": "Mark step number as complete (1-indexed)"},
                "notes": {"type": "string", "description": "Notes for the completed step"}
            }
        })
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        exit::execute(params)
    }
}

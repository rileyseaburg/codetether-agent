//! enter tool for the planning tools.
use super::*;
#[async_trait]
impl Tool for PlanEnterTool {
    fn id(&self) -> &str {
        "plan_enter"
    }

    fn name(&self) -> &str {
        "Enter Plan Mode"
    }

    fn description(&self) -> &str {
        "Enter planning mode with a goal and list of steps. Use before complex multi-step tasks."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "goal": {"type": "string", "description": "The overall goal to achieve"},
                "steps": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Ordered list of steps to complete the goal"
                }
            },
            "required": ["goal", "steps"]
        })
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        enter::execute(params)
    }
}

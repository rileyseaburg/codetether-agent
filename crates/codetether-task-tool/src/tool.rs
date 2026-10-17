//! tool for in-memory task tracking.
use super::*;
#[async_trait]
impl Tool for TaskTool {
    fn id(&self) -> &str {
        "task"
    }

    fn name(&self) -> &str {
        "Task Manager"
    }

    fn description(&self) -> &str {
        "Manage sub-tasks: create, query status, complete, or list tasks for tracking complex workflows."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        execute::execute(params).await
    }
}

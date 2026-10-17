//! tool for PRD construction and validation.
use super::*;
#[async_trait]
impl Tool for PrdTool {
    fn id(&self) -> &str {
        "prd"
    }

    fn name(&self) -> &str {
        "PRD Generator"
    }

    fn description(&self) -> &str {
        r#"Generate and validate structured PRDs (Product Requirements Documents) for complex tasks.

Actions:
- analyze: Analyze a task description and return what questions need answering
- generate: Generate a PRD JSON from provided answers
- validate: Validate a PRD (schema, dependencies, etc.)
- save: Save a PRD JSON to a file for ralph to execute
"#
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.dispatch(params).await
    }
}

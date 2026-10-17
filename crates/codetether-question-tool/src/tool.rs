//! tool for the question tool.
use super::*;
#[async_trait]
impl Tool for QuestionTool {
    fn id(&self) -> &str {
        "question"
    }

    fn name(&self) -> &str {
        "Ask Question"
    }

    fn description(&self) -> &str {
        "Ask the user questions with labeled choices, descriptions, multiple selection, and custom answers. Put recommended choices first. Custom answers are offered automatically."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::execute(args).await
    }
}

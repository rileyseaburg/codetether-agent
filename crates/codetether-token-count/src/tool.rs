//! tool for the token count tool.
use super::*;
#[async_trait]
impl Tool for TokenCountTool {
    fn id(&self) -> &str {
        "token_count"
    }

    fn name(&self) -> &str {
        "Token Count"
    }

    fn description(&self) -> &str {
        "token_count(paths: string[]) - Estimate token cost of files BEFORE reading them. \
         Use on large or unknown files; if large, read a range or use rlm."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::execute(args).await
    }
}

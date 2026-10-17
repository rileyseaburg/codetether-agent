//! tool for bounded grep searches.
use super::*;
#[async_trait]
impl Tool for GrepTool {
    fn id(&self) -> &str {
        "grep"
    }

    fn name(&self) -> &str {
        "Grep Search"
    }

    fn description(&self) -> &str {
        "grep(pattern: string, path?: string, is_regex?: bool, include?: string, limit?: int) - Search for text or regex patterns in files. Respects .gitignore by default."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::execute(args).await
    }
}

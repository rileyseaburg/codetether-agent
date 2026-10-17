//! tool for file comparisons.
use super::*;
#[async_trait]
impl Tool for DiffTool {
    fn id(&self) -> &str {
        "diff"
    }

    fn name(&self) -> &str {
        "Diff"
    }

    fn description(&self) -> &str {
        "diff(file1?: string, file2?: string, git?: bool, staged?: bool) - Compare two files or show git changes. Use git=true for uncommitted changes, staged=true for staged changes."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::execute(args).await
    }
}

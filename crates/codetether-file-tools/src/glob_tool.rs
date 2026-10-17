//! glob tool for core file tools.
use super::*;
#[async_trait]
impl Tool for GlobTool {
    fn id(&self) -> &str {
        "glob"
    }

    fn name(&self) -> &str {
        "Glob Search"
    }

    fn description(&self) -> &str {
        "glob(pattern: string, limit?: int) - Find files matching a glob pattern (e.g., **/*.rs, src/**/*.ts)"
    }
    fn parameters(&self) -> Value {
        glob_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        glob_search::execute(self, args).await
    }
}

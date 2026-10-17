//! tool for workspace code search.
use super::*;
#[async_trait]
impl Tool for CodeSearchTool {
    fn id(&self) -> &str {
        "codesearch"
    }

    fn name(&self) -> &str {
        "Code Search"
    }

    fn description(&self) -> &str {
        "Search for code patterns in the workspace. Supports regex."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.run(params).await
    }
}

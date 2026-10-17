//! tool for web search.
use super::*;
#[async_trait]
impl Tool for WebSearchTool {
    fn id(&self) -> &str {
        "websearch"
    }

    fn name(&self) -> &str {
        "Web Search"
    }

    fn description(&self) -> &str {
        "Search the web for information. Returns titles, URLs, and snippets."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.run(params).await
    }
}

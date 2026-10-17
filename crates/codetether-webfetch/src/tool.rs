//! tool for web content fetching.
use super::*;
#[async_trait]
impl Tool for WebFetchTool {
    fn id(&self) -> &str {
        "webfetch"
    }

    fn name(&self) -> &str {
        "Web Fetch"
    }

    fn description(&self) -> &str {
        "Fetch content from URL, convert HTML to markdown/text/html."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.run(params).await
    }
}

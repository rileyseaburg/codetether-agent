//! preview tool for filesystem inspection.
use super::*;
#[async_trait]
impl Tool for HeadTailTool {
    fn id(&self) -> &str {
        "headtail"
    }

    fn name(&self) -> &str {
        "Head/Tail"
    }

    fn description(&self) -> &str {
        "headtail(path: string, head?: int, tail?: int) - Quickly peek at the beginning and/or end of a file. Useful for understanding file structure without reading the entire file."
    }
    fn parameters(&self) -> Value {
        preview_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        preview::execute(args).await
    }
}

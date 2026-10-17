//! write tool for core file tools.
use super::*;
#[async_trait]
impl Tool for WriteTool {
    fn id(&self) -> &str {
        "write"
    }

    fn name(&self) -> &str {
        "Write File"
    }

    fn description(&self) -> &str {
        "write(path: string, content: string) - Write content to a file. Creates the file if it doesn't exist, or overwrites it."
    }
    fn parameters(&self) -> Value {
        write_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        write::execute(args).await
    }
}

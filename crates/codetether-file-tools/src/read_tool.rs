//! read tool for core file tools.
use super::*;
#[async_trait]
impl Tool for ReadTool {
    fn id(&self) -> &str {
        "read"
    }

    fn name(&self) -> &str {
        "Read File"
    }

    fn description(&self) -> &str {
        "read(path: string, offset?: int, limit?: int) - Read the contents of a file. Provide the file path to read."
    }
    fn parameters(&self) -> Value {
        read_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        read::execute(args).await
    }
}

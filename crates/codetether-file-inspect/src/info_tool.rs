//! info tool for filesystem inspection.
use super::*;
#[async_trait]
impl Tool for FileInfoTool {
    fn id(&self) -> &str {
        "fileinfo"
    }

    fn name(&self) -> &str {
        "File Info"
    }

    fn description(&self) -> &str {
        "fileinfo(path: string) - Get detailed information about a file: size, type, permissions, line count, encoding detection, and language."
    }
    fn parameters(&self) -> Value {
        info_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        info::execute(args).await
    }
}

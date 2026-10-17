//! list tool for core file tools.
use super::*;
#[async_trait]
impl Tool for ListTool {
    fn id(&self) -> &str {
        "list"
    }

    fn name(&self) -> &str {
        "List Directory"
    }

    fn description(&self) -> &str {
        "list(path: string) - List the contents of a directory."
    }
    fn parameters(&self) -> Value {
        list_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        list::execute(args).await
    }
}

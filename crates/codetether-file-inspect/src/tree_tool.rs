//! tree tool for filesystem inspection.
use super::*;
#[async_trait]
impl Tool for TreeTool {
    fn id(&self) -> &str {
        "tree"
    }

    fn name(&self) -> &str {
        "Directory Tree"
    }

    fn description(&self) -> &str {
        "tree(path: string, depth?: int, show_hidden?: bool, show_size?: bool) - Display a tree view of directory structure. Great for understanding project layout."
    }
    fn parameters(&self) -> Value {
        tree_schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        tree::execute(args).await
    }
}

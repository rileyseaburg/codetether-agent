//! LSP placeholder: patch tests do not start language servers.
use codetether_tool_core::{Tool, ToolResult};
pub(super) struct NoLsp;
#[async_trait::async_trait]
impl Tool for NoLsp {
    fn id(&self) -> &str {
        "lsp"
    }
    fn name(&self) -> &str {
        "Unused LSP"
    }
    fn description(&self) -> &str {
        "Unused in this test"
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({})
    }
    async fn execute(&self, _: serde_json::Value) -> anyhow::Result<ToolResult> {
        anyhow::bail!("LSP must not run")
    }
}

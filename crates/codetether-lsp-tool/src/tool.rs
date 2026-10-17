//! LSP tool metadata and execution entry point.
use super::{Host, LspTool};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<H: Host> Tool for LspTool<H> {
    fn id(&self) -> &str {
        "lsp"
    }

    fn name(&self) -> &str {
        "LSP Tool"
    }

    fn description(&self) -> &str {
        "Perform Language Server Protocol (LSP) operations such as go-to-definition, find-references, hover, document-symbol, workspace-symbol, diagnostics, and more. This tool enables AI agents to query language servers for code intelligence features. Supports rust-analyzer, typescript-language-server, pylsp, gopls, and clangd."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::execute(self, args).await
    }
}

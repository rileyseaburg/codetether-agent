//! Dispatch validated document operations and format their results.
use super::LspOperation;
use anyhow::Result;
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::Path;
pub(super) async fn run<C: ManagedClient>(
    client: &C,
    manager: &LspManager<C>,
    path: &Path,
    action: LspOperation,
    args: &Value,
    line: Option<u32>,
    column: Option<u32>,
) -> Result<ToolResult> {
    let result = match action {
        LspOperation::GoToDefinition => {
            super::actions::definition::run(client, path, line, column).await?
        }
        LspOperation::FindReferences => {
            super::actions::references::run(client, path, args, line, column).await?
        }
        LspOperation::Hover => super::actions::hover::run(client, path, line, column).await?,
        LspOperation::DocumentSymbol => super::actions::document::run(client, path).await?,
        LspOperation::GoToImplementation => {
            super::actions::implementation::run(client, path, line, column).await?
        }
        LspOperation::Completion => {
            super::actions::completion::run(client, path, line, column).await?
        }
        LspOperation::Diagnostics => {
            super::actions::diagnostics::run(client, manager, path).await?
        }
        LspOperation::WorkspaceSymbol => {
            return Ok(ToolResult::error(format!(
                "Action {} is handled separately",
                action.canonical_name()
            )));
        }
    };
    super::format::format_result(result)
}

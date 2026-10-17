//! Convert typed LSP responses to existing tool output text.
use anyhow::Result;
use codetether_lsp_core::LspActionResult;
use codetether_tool_core::ToolResult;
mod completion;
mod definition;
mod diagnostics;
mod document;
mod hover;
mod implementation;
mod references;
mod workspace;
pub(super) fn format_result(result: LspActionResult) -> Result<ToolResult> {
    let output = match result {
        LspActionResult::Definition { locations } => definition::format(locations),
        LspActionResult::References { locations } => references::format(locations),
        LspActionResult::Hover { contents, range } => hover::format(contents, range),
        LspActionResult::DocumentSymbols { symbols } => document::format(symbols),
        LspActionResult::WorkspaceSymbols { symbols } => workspace::format(symbols),
        LspActionResult::Implementation { locations } => implementation::format(locations),
        LspActionResult::Completion { items } => completion::format(items),
        LspActionResult::Diagnostics { diagnostics } => diagnostics::format(diagnostics),
        LspActionResult::Error { message } => {
            return Ok(ToolResult::error(message));
        }
    };
    Ok(ToolResult::success(output))
}

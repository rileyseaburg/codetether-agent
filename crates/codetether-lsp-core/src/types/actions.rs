//! LSP actions definitions and helpers.

use super::*;

/// LSP action result - unified response type for the tool
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LspActionResult {
    /// Go to definition result
    Definition { locations: Vec<LocationInfo> },
    /// Find references result
    References { locations: Vec<LocationInfo> },
    /// Hover result
    Hover {
        contents: String,
        range: Option<RangeInfo>,
    },
    /// Document symbols result
    DocumentSymbols { symbols: Vec<SymbolInfo> },
    /// Workspace symbols result
    WorkspaceSymbols { symbols: Vec<SymbolInfo> },
    /// Go to implementation result
    Implementation { locations: Vec<LocationInfo> },
    /// Completion result
    Completion { items: Vec<CompletionItemInfo> },
    /// Diagnostics result
    Diagnostics { diagnostics: Vec<DiagnosticInfo> },
    /// Error result
    Error { message: String },
}

//! Refuse `lsp` actions the connected server did not advertise.
//!
//! A diagnostics-only server (TetherScript's `tetherscript lsp` advertises just
//! `textDocumentSync`) would otherwise sit on a `hover` request until the
//! transport timeout. Checking `ServerCapabilities` first turns that into an
//! immediate, actionable tool error.

use lsp_types::ServerCapabilities;

use super::LspOperation;

/// `Err(reason)` when `caps` shows the server cannot serve `action`.
pub(super) fn check(action: LspOperation, caps: Option<&ServerCapabilities>) -> Result<(), String> {
    let Some(caps) = caps else {
        return Ok(()); // no handshake recorded; let the request decide
    };
    let supported = match action {
        LspOperation::GoToDefinition => caps.definition_provider.is_some(),
        LspOperation::FindReferences => caps.references_provider.is_some(),
        LspOperation::Hover => caps.hover_provider.is_some(),
        LspOperation::DocumentSymbol => caps.document_symbol_provider.is_some(),
        LspOperation::WorkspaceSymbol => caps.workspace_symbol_provider.is_some(),
        LspOperation::GoToImplementation => caps.implementation_provider.is_some(),
        LspOperation::Completion => caps.completion_provider.is_some(),
        LspOperation::Diagnostics => true, // publishDiagnostics needs no provider flag
    };
    if supported {
        return Ok(());
    }
    Err(format!(
        "language server does not support `{}` (advertises diagnostics only); \
         use action `diagnostics` for this file type",
        action.canonical_name()
    ))
}

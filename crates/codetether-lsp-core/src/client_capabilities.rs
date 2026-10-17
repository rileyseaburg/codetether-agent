//! LSP client capabilities needed by diagnostic-producing servers.

use lsp_types::{
    ClientCapabilities, PublishDiagnosticsClientCapabilities, TextDocumentClientCapabilities,
    WorkspaceClientCapabilities,
};

/// Build client capabilities used for diagnostic-producing servers.
///
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// Workspace-configuration and document-diagnostic support declarations.
/// # Examples
/// ```
/// assert!(codetether_lsp_core::client_capabilities::build().text_document.is_some());
/// ```
pub fn build() -> ClientCapabilities {
    ClientCapabilities {
        workspace: Some(WorkspaceClientCapabilities {
            configuration: Some(true),
            workspace_folders: Some(true),
            ..Default::default()
        }),
        text_document: Some(TextDocumentClientCapabilities {
            publish_diagnostics: Some(PublishDiagnosticsClientCapabilities {
                related_information: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

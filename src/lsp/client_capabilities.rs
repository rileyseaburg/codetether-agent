//! LSP client capabilities needed by diagnostic-producing servers.

use lsp_types::{
    ClientCapabilities, PublishDiagnosticsClientCapabilities, TextDocumentClientCapabilities,
    WorkspaceClientCapabilities,
};

pub(super) fn build() -> ClientCapabilities {
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

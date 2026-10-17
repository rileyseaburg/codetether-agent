//! document changes operations for language server clients.

use super::*;

impl LspClient {
    /// Update a text document
    pub async fn change_document(&self, path: &Path, content: &str) -> Result<()> {
        let uri = path_to_uri(path);
        let mut open_docs = self.open_documents.write().await;

        let version = open_docs.entry(uri.clone()).or_insert(0);
        *version += 1;

        let text_document = VersionedTextDocumentIdentifier {
            uri,
            version: *version,
        };

        let content_changes = vec![super::super::types::TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: content.to_string(),
        }];

        let params = DidChangeTextDocumentParams {
            text_document,
            content_changes,
        };

        self.transport
            .notify(
                "textDocument/didChange",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        debug!(path = %path.display(), version = *version, "Changed document");

        Ok(())
    }
}

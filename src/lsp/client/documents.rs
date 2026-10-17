//! documents operations for language server clients.

use super::*;

impl LspClient {
    /// Open a text document
    pub async fn open_document(&self, path: &Path, content: &str) -> Result<()> {
        let uri = path_to_uri(path);
        let language_id = super::super::document_language::id(path).to_string();

        let text_document = TextDocumentItem {
            uri: parse_uri(&uri)?,
            language_id,
            version: 1,
            text: content.to_string(),
        };

        let params = DidOpenTextDocumentParams { text_document };
        self.transport
            .notify("textDocument/didOpen", Some(serde_json::to_value(params)?))
            .await?;

        self.open_documents.write().await.insert(uri, 1);
        debug!(path = %path.display(), "Opened document");

        Ok(())
    }

    /// Close a text document
    #[allow(dead_code)]
    pub async fn close_document(&self, path: &Path) -> Result<()> {
        let uri = path_to_uri(path);

        let text_document = TextDocumentIdentifier {
            uri: parse_uri(&uri)?,
        };

        let params = DidCloseTextDocumentParams { text_document };
        self.transport
            .notify("textDocument/didClose", Some(serde_json::to_value(params)?))
            .await?;

        self.open_documents.write().await.remove(&uri);
        debug!(path = %path.display(), "Closed document");

        Ok(())
    }

    /// Ensure a document is open (open it if not already)
    pub(super) async fn ensure_document_open(&self, path: &Path) -> Result<()> {
        let uri = path_to_uri(path);
        if !self.open_documents.read().await.contains_key(&uri) {
            let content = tokio::fs::read_to_string(path).await?;
            self.open_document(path, &content).await?;
        }
        Ok(())
    }
}

//! symbols operations for language server clients.

use super::*;

impl LspClient {
    /// Get document symbols
    pub async fn document_symbols(&self, path: &Path) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = DocumentSymbolParams {
            text_document: TextDocumentIdentifier {
                uri: parse_uri(&uri)?,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        let response = self
            .transport
            .request(
                "textDocument/documentSymbol",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        parse_document_symbols_response(response)
    }

    /// Search workspace symbols
    pub async fn workspace_symbols(&self, query: &str) -> Result<LspActionResult> {
        let params = WorkspaceSymbolParams {
            query: query.to_string(),
        };

        let response = self
            .transport
            .request("workspace/symbol", Some(serde_json::to_value(params)?))
            .await?;

        parse_workspace_symbols_response(response)
    }
}

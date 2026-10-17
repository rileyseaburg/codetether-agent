//! hover operations for language server clients.

use super::*;

impl LspClient {
    /// Get hover information
    pub async fn hover(&self, path: &Path, line: u32, character: u32) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: parse_uri(&uri)?,
                },
                position: Position {
                    line: line.saturating_sub(1),
                    character: character.saturating_sub(1),
                },
            },
            work_done_progress_params: Default::default(),
        };

        let response = self
            .transport
            .request("textDocument/hover", Some(serde_json::to_value(params)?))
            .await?;

        parse_hover_response(response)
    }
}

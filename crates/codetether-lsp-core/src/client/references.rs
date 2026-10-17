//! references operations for language server clients.

use super::*;

impl LspClient {
    /// Find references
    pub async fn find_references(
        &self,
        path: &Path,
        line: u32,
        character: u32,
        include_declaration: bool,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = ReferenceParams {
            text_document: TextDocumentIdentifier {
                uri: parse_uri(&uri)?,
            },
            position: Position {
                line: line.saturating_sub(1),
                character: character.saturating_sub(1),
            },
            context: ReferenceContext {
                include_declaration,
            },
        };

        let response = self
            .transport
            .request(
                "textDocument/references",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        parse_location_response(response, "references")
    }
}

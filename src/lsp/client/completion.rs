//! completion operations for language server clients.

use super::*;

impl LspClient {
    /// Get code completions
    pub async fn completion(
        &self,
        path: &Path,
        line: u32,
        character: u32,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: parse_uri(&uri)?,
                },
                position: Position {
                    line: line.saturating_sub(1),
                    character: character.saturating_sub(1),
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: Some(CompletionContext {
                trigger_kind: CompletionTriggerKind::INVOKED,
                trigger_character: None,
            }),
        };

        let response = self
            .transport
            .request(
                "textDocument/completion",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        parse_completion_response(response)
    }
}

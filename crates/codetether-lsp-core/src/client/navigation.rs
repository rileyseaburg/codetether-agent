//! navigation operations for language server clients.

use super::*;

impl LspClient {
    /// Go to definition
    pub async fn go_to_definition(
        &self,
        path: &Path,
        line: u32,
        character: u32,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line.saturating_sub(1), "character": character.saturating_sub(1) },
        });

        let response = self
            .transport
            .request("textDocument/definition", Some(params))
            .await?;

        parse_location_response(response, "definition")
    }

    /// Go to implementation
    pub async fn go_to_implementation(
        &self,
        path: &Path,
        line: u32,
        character: u32,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line.saturating_sub(1), "character": character.saturating_sub(1) },
        });

        let response = self
            .transport
            .request("textDocument/implementation", Some(params))
            .await?;

        parse_location_response(response, "implementation")
    }
}

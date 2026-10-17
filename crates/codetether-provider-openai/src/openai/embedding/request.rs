//! request for the OpenAI-compatible transport.

use super::*;

impl OpenAIProvider {
    pub(in crate::openai) async fn embed_request(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse> {
        if request.inputs.is_empty() {
            return Ok(EmbeddingResponse {
                embeddings: Vec::new(),
                usage: Usage::default(),
            });
        }

        let url = format!("{}/embeddings", self.api_base.trim_end_matches('/'));
        let body = OpenAIEmbeddingRequest {
            model: request.model,
            input: request.inputs,
        };

        let mut request_builder = self.http.post(url);
        if let Some(api_key) = self.api_key.as_deref().filter(|key| !key.is_empty()) {
            request_builder = request_builder.bearer_auth(api_key);
        }
        let response = request_builder.json(&body).send().await?;

        parse_embedding_response(response, body.input.len()).await
    }
}

//! Streaming request preparation and SSE conversion.
use crate::*;
mod decode;
mod event;
mod line;
mod send;
impl VertexAnthropicProvider {
    pub(crate) async fn stream_request(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let access_token = self.get_access_token().await?;

        let body = Self::stream_body(&request);
        let url = self.build_model_url(&request.model);

        tracing::debug!(model = %request.model, "Vertex Anthropic streaming request");

        let response = self.send_stream(&url, &access_token, &body).await?;
        Ok(decode::decode(response))
    }
}

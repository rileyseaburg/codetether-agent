//! request for the OpenAI-compatible transport.

use super::*;

impl OpenAIProvider {
    pub(in crate::openai) async fn stream_request(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        tracing::debug!(
            provider = %self.provider_name,
            model = %request.model,
            message_count = request.messages.len(),
            "Starting streaming completion request"
        );

        if self.uses_reasoning_stream() {
            let body = self.reasoning_body(&request);
            return self.reasoning_stream(request, body).await;
        }

        let body = self.stream_body(&request)?;
        let stream = self.client.chat().create_stream(body).await?;

        Ok(stream
            .flat_map(|result| {
                let chunks = match result {
                    Ok(response) => typed_response_chunks(response),
                    Err(error) => vec![StreamChunk::Error(error.to_string())],
                };
                futures::stream::iter(chunks)
            })
            .boxed())
    }
}

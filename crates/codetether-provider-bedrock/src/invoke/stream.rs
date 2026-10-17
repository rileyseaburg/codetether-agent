//! Stream adapter: run a non-streaming InvokeModel completion and replay it
//! as a synthetic stream of [`StreamChunk`]s.

use super::stream_parts::push_part_chunks;
use crate::BedrockClient;
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, StreamChunk};
use futures::stream;

impl BedrockClient<'_> {
    ///
    /// # Arguments
    /// Accepts the request, resolved model ID, and application recovery handler.
    ///
    /// # Returns
    /// The completion result or its provider-neutral stream.
    ///
    /// # Errors
    /// Returns an error when request execution or response decoding fails.
    ///
    /// # Examples
    /// Construct [BedrockClient] and pass an implementation of
    /// [crate::recovery::CredentialRecovery] to supply application refresh.
    pub async fn invoke_model_stream_adapter(
        &self,
        request: &CompletionRequest,
        model_id: &str,
        recovery: &dyn crate::recovery::CredentialRecovery,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let response = self
            .complete_invoke_model(request, model_id, recovery)
            .await?;
        let mut chunks = Vec::new();
        for part in response.message.content {
            push_part_chunks(part, &mut chunks);
        }
        chunks.push(StreamChunk::Done {
            usage: Some(response.usage),
        });
        Ok(Box::pin(stream::iter(chunks)))
    }
}

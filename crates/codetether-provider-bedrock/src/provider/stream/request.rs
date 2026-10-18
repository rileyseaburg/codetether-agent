//! Delegate Bedrock execution to the client crate.
use crate::provider::BedrockProvider;
use anyhow::Result;
use codetether_provider_core::StreamChunk;
impl BedrockProvider {
    /// POST to `/model/{id}/converse-stream` and yield `StreamChunk`s as
    /// eventstream frames arrive.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the initial HTTP request fails or the
    /// server responds non-200. Per-frame decode errors are emitted as
    /// [`StreamChunk::Error`] but do not abort the stream.
    pub(in crate::provider) async fn converse_stream(
        &self,
        model_id: &str,
        body: Vec<u8>,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.bedrock_client()
            .converse_stream(model_id, body, self)
            .await
    }
}

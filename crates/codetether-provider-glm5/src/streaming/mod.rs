//! Streaming request and response orchestration.
use crate::*;
impl Glm5Provider {
    pub(crate) async fn streaming(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let (body, model) = Self::streaming_body(&request);
        tracing::debug!(
            model = %model,
            endpoint = %self.base_url,
            "GLM-5 FP8 streaming request"
        );

        let response = self
            .send(&body)
            .await
            .context("Failed to send streaming request to GLM-5 FP8 endpoint")?;

        let response = admit::admit(response).await?;

        Ok(decode::decode(response))
    }
}
mod admit;
mod decode;
mod finish;
mod line;
mod tools;

//! Streaming admission, retries, and event decoding.
use crate::*;
impl VertexGlmProvider {
    pub(crate) async fn streaming(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let mut access_token = self.get_access_token().await?;

        let body = Self::body(&request, true);
        tracing::debug!(model = %request.model, "Vertex GLM streaming request");

        let url = format!("{}/chat/completions", self.base_url);
        let mut last_err = String::new();

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                access_token = self.stream_backoff(attempt).await?;
            }

            let send_result = self.send(&url, &access_token, &body).await;

            let response = match send_result {
                Ok(r) => r,
                Err(e) if e.is_timeout() && attempt + 1 < MAX_RETRIES => {
                    tracing::warn!(error = %e, "Vertex GLM streaming request timed out");
                    last_err = format!("Request timed out: {e}");
                    continue;
                }
                Err(e) => anyhow::bail!("Failed to send streaming request to Vertex AI GLM: {e}"),
            };

            let Some(response) = admit::admit(response, attempt, &mut last_err).await? else {
                continue;
            };

            return Ok(decode::decode(response));
        }

        anyhow::bail!("Vertex AI GLM streaming failed after {MAX_RETRIES} attempts: {last_err}")
    }
}
mod admit;
mod decode;
mod finish;
mod line;
mod tools;

//! Buffered completion retry orchestration.
use crate::*;
impl VertexGlmProvider {
    pub(crate) async fn buffered(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let mut access_token = self.get_access_token().await?;

        let body = Self::body(&request, false);
        tracing::debug!(model = %request.model, "Vertex GLM request");

        let url = format!("{}/chat/completions", self.base_url);
        let mut last_err = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                access_token = self.buffered_backoff(attempt).await?;
            }

            let send_result = self.send(&url, &access_token, &body).await;

            let response = match send_result {
                Ok(r) => r,
                Err(e) if e.is_timeout() && attempt + 1 < MAX_RETRIES => {
                    tracing::warn!(error = %e, "Vertex GLM request timed out");
                    last_err = Some(format!("Request timed out: {e}"));
                    continue;
                }
                Err(e) => anyhow::bail!("Failed to send request to Vertex AI GLM: {e}"),
            };

            let Some(text) = read::read(response, attempt, &mut last_err).await? else {
                continue;
            };

            return response::parse(&text);
        }

        anyhow::bail!(
            "Vertex AI GLM request failed after {MAX_RETRIES} attempts: {}",
            last_err.unwrap_or_default()
        )
    }
}
mod buffered_backoff;
mod read;
mod send;
mod stream_backoff;

//! stream retry delay and token refresh.
use crate::*;
impl VertexGlmProvider {
    pub(crate) async fn stream_backoff(&self, attempt: u32) -> Result<String> {
        let backoff = Duration::from_millis(1000 * 2u64.pow(attempt - 1));
        tracing::warn!(
            attempt,
            backoff_ms = backoff.as_millis() as u64,
            "Vertex GLM streaming retrying after transient error"
        );
        tokio::time::sleep(backoff).await;
        self.get_access_token().await
    }
}

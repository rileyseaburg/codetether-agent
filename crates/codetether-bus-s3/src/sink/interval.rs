//! interval for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    pub(super) async fn flush_on_interval(
        &self,
        batch: &mut Vec<BusEnvelope>,
        batch_start: &mut Option<String>,
    ) {
        if !batch.is_empty() {
            if let Err(e) = self.flush_batch(batch, batch_start).await {
                error!(error = %e, "Failed to flush batch on interval");
            }
            // Yield to allow LLM requests priority access to executor
            task::yield_now().await;
        }
    }
}

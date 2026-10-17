//! receive for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    pub(super) async fn receive(
        &self,
        result: std::result::Result<BusEnvelope, broadcast::error::RecvError>,
        batch: &mut Vec<BusEnvelope>,
        batch_start: &mut Option<String>,
    ) -> bool {
        match result {
            Ok(envelope) => {
                if batch_start.is_none() {
                    *batch_start = Some(envelope.timestamp.to_rfc3339());
                }
                batch.push(envelope);

                if batch.len() >= self.config.batch_size {
                    if let Err(e) = self.flush_batch(batch, batch_start).await {
                        error!(error = %e, "Failed to flush batch");
                    }
                    // Yield to allow LLM requests priority access to executor
                    task::yield_now().await;
                }
            }
            Err(broadcast::error::RecvError::Lagged(n)) => {
                warn!(skipped = n, "Bus S3 sink lagged, some messages dropped");
            }
            Err(broadcast::error::RecvError::Closed) => {
                info!("Bus channel closed, shutting down S3 sink");
                if !batch.is_empty()
                    && let Err(e) = self.flush_batch(batch, batch_start).await
                {
                    error!(error = %e, "Failed to flush final batch");
                }
                return true;
            }
        }
        false
    }
}

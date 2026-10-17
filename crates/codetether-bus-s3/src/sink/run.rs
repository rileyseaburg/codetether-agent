//! run for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    /// Subscribe until the bus closes, flushing on capacity and interval.
    pub async fn run(mut self) -> Result<()> {
        self.ensure_bucket().await?;

        info!(
            bucket = %self.config.bucket,
            prefix = %self.config.prefix,
            batch_size = self.config.batch_size,
            flush_secs = self.config.flush_interval_secs,
            "Bus S3 sink started (JSONL training record format)"
        );

        let mut batch: Vec<BusEnvelope> = Vec::with_capacity(self.config.batch_size);
        let mut batch_start: Option<String> = None;
        let mut flush_interval = tokio::time::interval(std::time::Duration::from_secs(
            self.config.flush_interval_secs,
        ));

        loop {
            tokio::select! {
                result = self.rx.recv() => {
                    if self.receive(result, &mut batch, &mut batch_start).await {
                        return Ok(());
                    }
                }
                _ = flush_interval.tick() => {
                    self.flush_on_interval(&mut batch, &mut batch_start).await;
                }
            }
        }
    }
}

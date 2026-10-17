//! flush for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    /// Flush the current batch to S3 as JSONL (one training record per line).
    ///
    /// Heartbeats and duplicate full-output telemetry are filtered out. Each
    /// retained envelope becomes one chronological `TrainingRecord` JSON line.
    pub(super) async fn flush_batch(
        &self,
        batch: &mut Vec<BusEnvelope>,
        batch_start: &mut Option<String>,
    ) -> Result<()> {
        if batch.is_empty() {
            return Ok(());
        }

        let _start_time = batch_start
            .take()
            .unwrap_or_else(|| Utc::now().to_rfc3339());
        let envelopes = std::mem::take(batch);

        // Build JSONL: buffer tool requests/responses by agent step and
        // correlation id so separate turns do not merge when a sender reuses
        // the same step number. Legacy producers with `None` correlation ids
        // still group together compatibly.
        let lines = serialize_envelopes(&envelopes);
        if lines.is_empty() {
            return Ok(());
        }

        let count = lines.len();
        let jsonl = lines.join("\n");

        // S3 key: prefix/v2/YYYY/MM/DD/HH/batch_YYYYMMDDTHHMMSS_uuid.jsonl
        let now = Utc::now();
        let s3_key = build_s3_key(&self.config.prefix, now);

        self.upload(s3_key, jsonl, count).await
    }
}

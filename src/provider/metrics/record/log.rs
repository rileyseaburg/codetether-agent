//! Structured provider metric logging.

use crate::telemetry::ProviderRequestRecord;

/// Writes one normalized provider completion log entry.
pub(super) fn write(record: &ProviderRequestRecord, message: &str) {
    tracing::info!(provider = %record.provider, model = %record.model,
        latency_ms = record.latency_ms, ttft_ms = record.ttft_ms,
        input_tokens = record.input_tokens, output_tokens = record.output_tokens,
        tps = format!("{:.1}", record.tokens_per_second()), "{message}");
}

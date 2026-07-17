//! Construction of normalized provider telemetry records.

use super::super::super::Usage;
use crate::telemetry::ProviderRequestRecord;

/// Builds one provider record from timing and optional usage data.
pub(super) fn new(
    provider: &str,
    model: &str,
    start: std::time::Instant,
    ttft_ms: Option<u64>,
    usage: Option<&Usage>,
    success: bool,
) -> ProviderRequestRecord {
    let (input, output) = usage.map_or((0, 0), |value| {
        (value.prompt_tokens as u64, value.completion_tokens as u64)
    });
    ProviderRequestRecord {
        provider: provider.into(),
        model: model.into(),
        timestamp: chrono::Utc::now(),
        prompt_tokens: input,
        completion_tokens: output,
        input_tokens: input,
        output_tokens: output,
        latency_ms: start.elapsed().as_millis() as u64,
        ttft_ms,
        success,
    }
}

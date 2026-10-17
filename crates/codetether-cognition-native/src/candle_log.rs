//! Native diagnostics retaining application log targets.
use super::candle_output::Counts;
use std::time::Instant;

pub(super) fn no_eos() {
    tracing::warn!(target: "codetether_agent::cognition::thinker::candle::candle_new_generic", "No EOS tokens in tokenizer; generation stops on max token limit");
}

pub(super) fn success(model_label: &str, started_at: Instant, counts: &Counts) {
    tracing::debug!(target: "codetether_agent::cognition::thinker::candle::candle_think",
        model = %model_label,
        latency_ms = started_at.elapsed().as_millis(),
        prompt_tokens = counts.prompt,
        cache_read_tokens = counts.cache_read,
        cache_write_tokens = counts.cache_write,
        "candle thinker generated thought"
    );
}

#[cfg(feature = "candle-cuda")]
pub(super) fn selected(ordinal: usize) {
    tracing::info!(target: "codetether_agent::cognition::thinker::candle::candle_device",
        ordinal = ordinal,
        "Candle thinker selected CUDA device"
    );
}

#[cfg(feature = "candle-cuda")]
pub(super) fn unavailable(error: &anyhow::Error) {
    tracing::warn!(target: "codetether_agent::cognition::thinker::candle::candle_device", %error, "CUDA unavailable for Candle thinker, falling back to CPU");
}

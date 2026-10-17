//! Token accounting and [`ThinkerOutput`] assembly for Candle inference.

use super::ThinkerOutput;
use super::candle_decoded::Decoded;

/// Token counts for one Candle inference request.
pub(super) struct Counts {
    pub prompt: u32,
    pub cache_read: u32,
    pub cache_write: u32,
}

/// Assemble the public output from decode results and token counts.
pub(super) fn build(
    model_label: &str,
    text: String,
    decoded: Decoded,
    counts: Counts,
) -> ThinkerOutput {
    let completion = decoded.generated.len() as u32;
    ThinkerOutput {
        model: model_label.to_string(),
        finish_reason: Some(decoded.finish_reason),
        text,
        prompt_tokens: Some(counts.prompt),
        completion_tokens: Some(completion),
        total_tokens: Some(counts.prompt + completion),
        cache_read_tokens: Some(counts.cache_read),
        cache_write_tokens: Some(counts.cache_write),
    }
}

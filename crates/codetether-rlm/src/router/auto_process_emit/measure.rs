//! Measure token output, elapsed time, and compression ratio.

use crate::chunker::RlmChunker;
use std::time::Instant;
pub(super) fn measure(answer: &str, input_tokens: usize, start: Instant) -> (usize, u64, f64) {
    let output_tokens = RlmChunker::estimate_tokens(answer);
    let elapsed = start.elapsed().as_millis() as u64;
    let ratio = if output_tokens == 0 {
        0.0
    } else {
        output_tokens as f64 / input_tokens as f64
    };
    (output_tokens, elapsed, ratio)
}

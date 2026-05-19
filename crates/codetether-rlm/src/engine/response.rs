//! RLM result construction for the engine.

use std::time::Instant;
use uuid::Uuid;

use crate::chunker::RlmChunker;
use crate::result::RlmResult;
use crate::stats::RlmStats;

use super::{evidence::Evidence, trace};

/// Build a successful engine result.
pub fn success(
    answer: String,
    input_tokens: usize,
    start: Instant,
    trace_id: Uuid,
    evidence: Option<&Evidence>,
) -> RlmResult {
    let output_tokens = RlmChunker::estimate_tokens(&answer);
    let ratio = if input_tokens == 0 {
        0.0
    } else {
        output_tokens as f64 / input_tokens as f64
    };
    RlmResult {
        processed: format!(
            "[RLM engine: {input_tokens} -> {output_tokens} tokens | evidence-first]\n\n{answer}"
        ),
        stats: RlmStats {
            input_tokens,
            output_tokens,
            iterations: 1,
            subcalls: 0,
            elapsed_ms: start.elapsed().as_millis() as u64,
            compression_ratio: ratio,
        },
        success: true,
        error: None,
        trace: Some(trace::build(
            &answer,
            input_tokens.max(4_096),
            output_tokens,
            evidence,
        )),
        trace_id: Some(trace_id),
    }
}

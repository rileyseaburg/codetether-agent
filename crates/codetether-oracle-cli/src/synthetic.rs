//! Construct the synthetic analysis submitted for deterministic validation.
use codetether_rlm::{RlmAnalysisResult, RlmChunker, RlmStats, SubQuery};

pub(super) fn build(
    query: &str,
    source: &str,
    payload: &str,
    source_path: Option<String>,
) -> RlmAnalysisResult {
    let synthetic = RlmAnalysisResult {
        answer: payload.to_string(),
        iterations: 1,
        sub_queries: vec![SubQuery {
            query: query.to_string(),
            context_slice: source_path.clone(),
            response: payload.to_string(),
            tokens_used: 0,
        }],
        stats: RlmStats {
            input_tokens: RlmChunker::estimate_tokens(&source),
            output_tokens: RlmChunker::estimate_tokens(&payload),
            iterations: 1,
            subcalls: 0,
            elapsed_ms: 0,
            compression_ratio: 1.0,
        },
    };

    synthetic
}

//! Fallback result builders.

use crate::chunker::RlmChunker;
use crate::result::RlmResult;
use crate::stats::RlmStats;
use super::truncate::smart_truncate;

/// Build a fallback `RlmResult` (truncation only, no LLM call).
pub fn fallback_result(
    output: &str, tool_id: &str, tool_args: &serde_json::Value, input_tokens: usize,
) -> RlmResult {
    let (truncated, _, _) = smart_truncate(output, tool_id, tool_args, 8000);
    let out_tok = RlmChunker::estimate_tokens(&truncated);
    RlmResult {
        processed: format!("[RLM processing failed, showing truncated output]\n\n{truncated}"),
        stats: RlmStats {
            input_tokens, output_tokens: out_tok, iterations: 0, subcalls: 0,
            elapsed_ms: 0, compression_ratio: input_tokens as f64 / out_tok.max(1) as f64,
        },
        success: false, error: Some("Model call failed".into()),
        trace: None, trace_id: None,
    }
}

/// Enhanced fallback — structured excerpt depending on tool type.
pub fn enhanced_fallback(
    output: &str, tool_id: &str, tool_args: &serde_json::Value, input_tokens: usize,
) -> String {
    if tool_id == "session_context" {
        super::fallback_context::session_context_fallback(output, input_tokens)
    } else {
        let (truncated, _, _) = smart_truncate(output, tool_id, tool_args, 8000);
        format!("## Fallback Summary\n*RLM processing failed - showing structured excerpt*\n\n{truncated}")
    }
}

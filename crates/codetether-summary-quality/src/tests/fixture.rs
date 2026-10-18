use codetether_rlm::{RlmResult, RlmStats};

pub(super) fn rlm_result(
    success: bool,
    body: &str,
    input_tokens: usize,
    output_tokens: usize,
) -> RlmResult {
    RlmResult {
        processed: body.to_string(),
        stats: RlmStats {
            input_tokens,
            output_tokens,
            ..Default::default()
        },
        success,
        error: None,
        trace: None,
        trace_id: None,
    }
}

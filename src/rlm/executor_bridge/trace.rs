//! Trace capture for executor bridge.

use super::RlmExecutor;

pub(super) fn record_trace(
    exec: &mut RlmExecutor,
    query: &str,
    answer: &str,
    result: &crate::rlm::RlmResult,
) {
    exec.trace_steps.clear();
    exec.context_trace = result
        .trace
        .clone()
        .unwrap_or_else(|| crate::rlm::context_trace::ContextTrace::new(32_768));
    exec.trace_steps.push(crate::rlm::oracle::TraceStep {
        iteration: result.stats.iterations.max(1),
        action: format!("rlm_engine({})", clipped(query, 120)),
        output: clipped(answer, 240),
    });
}

fn clipped(text: &str, max: usize) -> String {
    let mut out: String = text.chars().take(max).collect();
    if text.chars().count() > max {
        out.push_str("...");
    }
    out
}

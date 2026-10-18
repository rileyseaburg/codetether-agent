//! Borrow disjoint application metric fields for the shared reset policy.
use crate::tui::app::state::AppState;
use codetether_chat_transcript::sync::Metrics;
pub(super) fn fields(state: &mut AppState) -> Metrics<'_> {
    Metrics {
        request: &mut state.request_timing,
        tool: &mut state.tool_timing,
        model: &mut state.last_completion.model,
        latency: &mut state.last_completion.latency_ms,
        prompt_tokens: &mut state.last_completion.prompt_tokens,
        output_tokens: &mut state.last_completion.output_tokens,
    }
}

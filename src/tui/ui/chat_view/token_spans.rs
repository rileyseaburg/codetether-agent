//! Application-value adapters for shared token status spans.
use crate::tui::app::state::App;
pub use codetether_token_display::token_spans::push_token_spans;
use ratatui::text::Span;
/// Append the live throughput badge when streaming.
pub fn push_throughput_span(spans: &mut Vec<Span<'static>>, app: &App) {
    codetether_token_display::token_spans::push_throughput_span(
        spans,
        app.state.streaming_tok_per_sec(),
        app.state.processing,
    );
}
/// Append the most recently estimated context usage.
pub fn push_context_budget_span(spans: &mut Vec<Span<'static>>, app: &App) {
    codetether_token_display::token_spans::push_context_budget_span(
        spans,
        app.state.context_used,
        app.state.context_budget,
    );
}

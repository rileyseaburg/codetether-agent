//! Application-value adapters for shared context gauges.
use crate::tui::app::state::App;
pub use codetether_token_display::context_gauge::context_gauge_heat;
use ratatui::text::Span;
/// Build a color-aware context-budget gauge.
pub fn context_gauge_spans(app: &App) -> Option<Vec<Span<'static>>> {
    codetether_token_display::context_gauge::context_gauge_spans(
        app.state.context_health.used_tokens,
        app.state.context_health.token_budget,
    )
}
/// Build the single-span context-budget gauge.
pub fn context_gauge_span(app: &App) -> Option<Span<'static>> {
    codetether_token_display::context_gauge::context_gauge_span(
        app.state.context_health.used_tokens,
        app.state.context_health.token_budget,
    )
}

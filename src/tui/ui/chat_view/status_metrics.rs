//! Metric span assembly for the bottom status bar.
//!
//! Collects processing badge, token usage, context gauge, throughput
//! sparkline, and status text into a single span sequence.

use ratatui::text::Span;

use super::status_text::status_text_span;
use super::token_spans::{push_throughput_span, push_token_spans};
use crate::tui::app::state::App;

/// Assemble the metric spans (tokens, gauge, throughput, status text).
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::status_metrics::metric_spans;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let spans = metric_spans(app);
/// assert!(!spans.is_empty());
/// # }
/// ```
pub fn metric_spans(app: &App) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    if let Some(badge) = super::processing_badge::processing_badge(app) {
        spans.push(badge);
        spans.push(Span::raw(" | "));
    }
    push_token_spans(&mut spans);
    if let Some(gauge) = super::context_gauge::context_gauge_spans(app) {
        spans.push(Span::raw(" | "));
        spans.extend(gauge);
    }
    push_throughput_span(&mut spans, app);
    if let Some(spark) = super::throughput_sparkline::sparkline_spans(app) {
        spans.push(Span::raw(" "));
        spans.extend(spark);
    }
    spans.push(Span::raw(" | "));
    spans.push(status_text_span(app));
    spans
}

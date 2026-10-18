//! Native providers for shared metric ordering.
use super::{
    status_text::status_text_span,
    token_spans::{push_throughput_span, push_token_spans},
};
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Assemble metric spans while retaining native telemetry sampling.
///
/// # Arguments
/// * `app` - Application values supplied to the shared renderer.
///
/// # Returns
/// Owned status display spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _display = codetether_agent::tui::ui::chat_view::status_metrics::metric_spans(app);
/// # }
/// ```
pub fn metric_spans(app: &App) -> Vec<Span<'static>> {
    codetether_chat_status::metric_spans(&codetether_chat_status::MetricProviders {
        state: app,
        processing: super::processing_badge::processing_badge,
        tokens: push_token_spans,
        context: super::context_gauge::context_gauge_spans,
        throughput: push_throughput_span,
        sparkline: super::throughput_sparkline::sparkline_spans,
        status: status_text_span,
    })
}

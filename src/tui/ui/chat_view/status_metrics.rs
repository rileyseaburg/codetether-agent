//! Native providers for shared metric ordering.
use super::{
    status_text::status_text_span,
    token_spans::{push_throughput_span, push_token_spans},
};
use crate::tui::app::state::App;
use ratatui::text::Span;
impl codetether_chat_status::MetricSource for App {
    fn processing_badge(&self) -> Option<Span<'static>> {
        super::processing_badge::processing_badge(self)
    }
    fn push_token_spans(&self, spans: &mut Vec<Span<'static>>) {
        push_token_spans(spans)
    }
    fn context_gauge_spans(&self) -> Option<Vec<Span<'static>>> {
        super::context_gauge::context_gauge_spans(self)
    }
    fn push_throughput_span(&self, spans: &mut Vec<Span<'static>>) {
        push_throughput_span(spans, self)
    }
    fn sparkline_spans(&self) -> Option<Vec<Span<'static>>> {
        super::throughput_sparkline::sparkline_spans(self)
    }
    fn status_text_span(&self) -> Span<'static> {
        status_text_span(self)
    }
}
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
    codetether_chat_status::metric_spans(app)
}

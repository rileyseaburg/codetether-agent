//! Processing, token, context, throughput and status component order.
use ratatui::text::Span;

/// Assemble metrics while retaining component sampling order.
///
/// # Arguments
/// * `source` - Provider of live metric components.
///
/// # Returns
/// Owned metric spans with separators.
///
/// # Examples
/// ```
/// # fn example<T: codetether_chat_status::MetricSource>(source: &T) {
/// let _metrics = codetether_chat_status::metric_spans(source);
/// # }
/// ```
pub fn metric_spans(source: &impl crate::MetricSource) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    if let Some(badge) = source.processing_badge() {
        spans.push(badge);
        spans.push(Span::raw(" | "));
    }
    source.push_token_spans(&mut spans);
    if let Some(gauge) = source.context_gauge_spans() {
        spans.push(Span::raw(" | "));
        spans.extend(gauge);
    }
    source.push_throughput_span(&mut spans);
    if let Some(spark) = source.sparkline_spans() {
        spans.push(Span::raw(" "));
        spans.extend(spark);
    }
    spans.push(Span::raw(" | "));
    spans.push(source.status_text_span());
    spans
}

//! Lazy metric callbacks over a frontend's borrowed state.
use crate::MetricSource;
use ratatui::text::Span;

/// Connects telemetry rendering functions without sampling them in advance.
///
/// # Examples
/// ```
/// let source = codetether_chat_status::MetricProviders {
///     state: &(), processing: |_| None, tokens: |_| {}, context: |_| None,
///     throughput: |_, _| {}, sparkline: |_| None,
///     status: |_| ratatui::text::Span::raw("ready"),
/// };
/// let spans = codetether_chat_status::metric_spans(&source);
/// assert!(spans.iter().any(|span| span.content == "ready"));
/// ```
pub struct MetricProviders<'a, S> {
    /// Live frontend state borrowed for the duration of rendering.
    pub state: &'a S,
    /// Active processing indicator.
    pub processing: fn(&S) -> Option<Span<'static>>,
    /// Global token telemetry sampled into the output spans.
    pub tokens: fn(&mut Vec<Span<'static>>),
    /// Current context-budget display.
    pub context: fn(&S) -> Option<Vec<Span<'static>>>,
    /// Streaming throughput sampled into the output spans.
    pub throughput: fn(&mut Vec<Span<'static>>, &S),
    /// Streaming throughput history display.
    pub sparkline: fn(&S) -> Option<Vec<Span<'static>>>,
    /// Current frontend status text.
    pub status: fn(&S) -> Span<'static>,
}

impl<S> MetricSource for MetricProviders<'_, S> {
    fn processing_badge(&self) -> Option<Span<'static>> {
        (self.processing)(self.state)
    }
    fn push_token_spans(&self, spans: &mut Vec<Span<'static>>) {
        (self.tokens)(spans)
    }
    fn context_gauge_spans(&self) -> Option<Vec<Span<'static>>> {
        (self.context)(self.state)
    }
    fn push_throughput_span(&self, spans: &mut Vec<Span<'static>>) {
        (self.throughput)(spans, self.state)
    }
    fn sparkline_spans(&self) -> Option<Vec<Span<'static>>> {
        (self.sparkline)(self.state)
    }
    fn status_text_span(&self) -> Span<'static> {
        (self.status)(self.state)
    }
}

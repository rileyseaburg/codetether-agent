//! Application-supplied metric rendering and sample updates.
use ratatui::text::Span;
/// Supplies metric components in their original sampling order.
///
/// # Implementors
/// Terminal hosts bridge their processing state, telemetry, and context budget.
pub trait MetricSource {
    /// Supply processing badge when requested by the assembler.
    fn processing_badge(&self) -> Option<Span<'static>>;
    /// Supply push token spans when requested by the assembler.
    fn push_token_spans(&self, spans: &mut Vec<Span<'static>>);
    /// Supply context gauge spans when requested by the assembler.
    fn context_gauge_spans(&self) -> Option<Vec<Span<'static>>>;
    /// Supply push throughput span when requested by the assembler.
    fn push_throughput_span(&self, spans: &mut Vec<Span<'static>>);
    /// Supply sparkline spans when requested by the assembler.
    fn sparkline_spans(&self) -> Option<Vec<Span<'static>>>;
    /// Supply status text span when requested by the assembler.
    fn status_text_span(&self) -> Span<'static>;
}

//! Application-supplied badge components consumed in display order.
use ratatui::text::Span;
/// Supplies rendered badge components without exposing application state.
///
/// # Implementors
/// Terminal hosts provide the current values for their session and workers.
pub trait BadgeSource {
    /// Supply auto apply spans at the time it is rendered.
    fn auto_apply_spans(&self) -> Vec<Span<'static>>;
    /// Supply yolo badge at the time it is rendered.
    fn yolo_badge(&self) -> Option<Span<'static>>;
    /// Supply trust status badge span at the time it is rendered.
    fn trust_status_badge_span(&self) -> Option<Span<'static>>;
    /// Supply bus status badge span at the time it is rendered.
    fn bus_status_badge_span(&self) -> Span<'static>;
    /// Supply pending images badge at the time it is rendered.
    fn pending_images_badge(&self) -> Option<Span<'static>>;
    /// Supply latency badge spans at the time it is rendered.
    fn latency_badge_spans(&self) -> Option<Vec<Span<'static>>>;
    /// Supply turn latency badge at the time it is rendered.
    fn turn_latency_badge(&self) -> Option<Span<'static>>;
    /// Supply agent rail spans at the time it is rendered.
    fn agent_rail_spans(&self) -> Vec<Span<'static>>;
}

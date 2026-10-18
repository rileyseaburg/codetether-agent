//! Native providers for ordered shared status badges.
use super::{
    auto_apply::auto_apply_spans, images_badge::pending_images_badge,
    turn_badge::turn_latency_badge, yolo_badge::yolo_badge,
};
use crate::tui::app::state::App;
use crate::tui::ui::{
    status_bar::{bus_status_badge_span, latency_badge_spans},
    trust_status::trust_status_badge_span,
};
use ratatui::text::Span;
impl codetether_chat_status::BadgeSource for App {
    fn auto_apply_spans(&self) -> Vec<Span<'static>> {
        auto_apply_spans(self)
    }
    fn yolo_badge(&self) -> Option<Span<'static>> {
        yolo_badge(self)
    }
    fn trust_status_badge_span(&self) -> Option<Span<'static>> {
        trust_status_badge_span()
    }
    fn bus_status_badge_span(&self) -> Span<'static> {
        bus_status_badge_span(self)
    }
    fn pending_images_badge(&self) -> Option<Span<'static>> {
        pending_images_badge(self)
    }
    fn latency_badge_spans(&self) -> Option<Vec<Span<'static>>> {
        latency_badge_spans(self)
    }
    fn turn_latency_badge(&self) -> Option<Span<'static>> {
        turn_latency_badge(self)
    }
    fn agent_rail_spans(&self) -> Vec<Span<'static>> {
        super::agent_rail::agent_rail_spans(self)
    }
}
/// Assemble badges from live application providers.
///
/// # Arguments
/// * `app` - Application values supplied to the shared renderer.
/// * `session_label` - Current session label.
///
/// # Returns
/// Owned status display spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _display = codetether_agent::tui::ui::chat_view::badges::badge_spans(app, "session");
/// # }
/// ```
pub fn badge_spans(app: &App, session_label: &str) -> Vec<Span<'static>> {
    codetether_chat_status::badge_spans(app, session_label)
}

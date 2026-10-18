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
    codetether_chat_status::badge_spans(
        &codetether_chat_status::BadgeProviders {
            state: app,
            auto_apply: auto_apply_spans,
            yolo: yolo_badge,
            trust: trust_status_badge_span,
            bus: bus_status_badge_span,
            images: pending_images_badge,
            latency: latency_badge_spans,
            turn: turn_latency_badge,
            agents: super::agent_rail::agent_rail_spans,
        },
        session_label,
    )
}

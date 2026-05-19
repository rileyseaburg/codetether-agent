//! Status-bar badge sequence assembly.

use ratatui::text::Span;

use crate::tui::app::state::App;
use crate::tui::ui::status_bar::{bus_status_badge_span, latency_badge_spans};

use super::auto_apply::auto_apply_spans;
use super::images_badge::pending_images_badge;
use super::status_hints::session_label_spans;
use super::turn_badge::turn_latency_badge;

/// Assemble session, bus, image, request, and turn latency badges.
pub fn badge_spans(app: &App, session_label: &str) -> Vec<Span<'static>> {
    let mut spans = session_label_spans(session_label);
    spans.extend(auto_apply_spans(app));
    spans.push(Span::raw(" | "));
    spans.push(bus_status_badge_span(app));
    push_optional_badge(&mut spans, pending_images_badge(app));
    if let Some(latency) = latency_badge_spans(app) {
        spans.push(Span::raw(" | "));
        spans.extend(latency);
    }
    push_optional_badge(&mut spans, turn_latency_badge(app));
    spans
}

fn push_optional_badge(spans: &mut Vec<Span<'static>>, badge: Option<Span<'static>>) {
    if let Some(badge) = badge {
        spans.push(Span::raw(" | "));
        spans.push(badge);
    }
}

//! Bottom status-line span assembly.
//!
//! [`build_status_spans`] collects keybinding hints, auto-apply badge,
//! bus status, latency, token usage, and status text into a single
//! [`Span`] vector for the status bar.

use ratatui::text::Span;

use crate::tui::app::state::App;
use crate::tui::ui::status_bar::{bus_status_badge_span, latency_badge_spans};

use super::auto_apply::auto_apply_spans;
use super::images_badge::pending_images_badge;
use super::status_hints::header_spans;
use super::status_text::status_text_span;
use super::token_spans::push_token_spans;

/// Assemble all status-bar spans in display order.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::status::build_status_spans;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let spans = build_status_spans(app, "test-session");
/// assert!(!spans.is_empty());
/// # }
/// ```
pub fn build_status_spans(app: &App, session_label: &str) -> Vec<Span<'static>> {
    let mut spans = header_spans(session_label);
    spans.extend(auto_apply_spans(app));
    spans.push(Span::raw(" | "));
    spans.push(bus_status_badge_span(app));
    if let Some(badge) = pending_images_badge(app) {
        spans.push(Span::raw(" | "));
        spans.push(badge);
    }
    if let Some(latency) = latency_badge_spans(app) {
        spans.push(Span::raw(" | "));
        spans.extend(latency);
    }
    spans.push(Span::raw(" | "));
    push_token_spans(&mut spans);
    spans.push(Span::raw(" | "));
    spans.push(status_text_span(app));
    spans
}

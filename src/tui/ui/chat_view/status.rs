//! Bottom status-line span assembly.
//!
//! [`build_status_lines`] collects keybinding hints, session label,
//! auto-apply badge, bus status, latency, token usage, and status
//! text into one or more [`Line`]s for the status bar. When the
//! terminal is too narrow to show everything on one row, the
//! content is stacked across three lines.

use ratatui::text::{Line, Span};

use super::badges::badge_spans;
use super::status_hints::keybinding_spans;
use super::status_text::status_text_span;
use super::token_spans::push_token_spans;
use crate::tui::app::state::App;

/// Width below which the status bar stacks across multiple rows.
pub const STACK_WIDTH_THRESHOLD: u16 = 180;

/// Assemble the status bar as one or more lines.
///
/// Returns a single line when `width >= STACK_WIDTH_THRESHOLD`,
/// otherwise stacks the content across three lines:
/// 1. keybinding hints
/// 2. session / auto-apply / bus / images / latency badges
/// 3. token usage / status text
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::status::build_status_lines;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let lines = build_status_lines(app, "test-session", 120);
/// assert!(!lines.is_empty());
/// # }
/// ```
pub fn build_status_lines(app: &App, session_label: &str, width: u16) -> Vec<Line<'static>> {
    let hints = keybinding_spans();
    let badges = badge_spans(app, session_label);
    let metrics = metric_spans(app);

    if width >= STACK_WIDTH_THRESHOLD {
        let mut combined = hints;
        combined.push(Span::raw(" | "));
        combined.extend(badges);
        combined.push(Span::raw(" | "));
        combined.extend(metrics);
        vec![Line::from(combined)]
    } else {
        vec![Line::from(hints), Line::from(badges), Line::from(metrics)]
    }
}

/// Assemble all status-bar spans in display order as a single flat row.
///
/// Retained for backward compatibility; prefer [`build_status_lines`].
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
    let mut spans = keybinding_spans();
    spans.push(Span::raw(" | "));
    spans.extend(badge_spans(app, session_label));
    spans.push(Span::raw(" | "));
    spans.extend(metric_spans(app));
    spans
}

fn metric_spans(app: &App) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    push_token_spans(&mut spans);
    spans.push(Span::raw(" | "));
    spans.push(status_text_span(app));
    spans
}

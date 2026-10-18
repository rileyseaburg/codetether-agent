//! Bottom status-line span assembly.
//!
//! [`build_status_lines`] collects keybinding hints, session label,
//! auto-apply badge, bus status, latency, token usage, and status
//! text into one or more [`Line`]s for the status bar. When the
//! terminal is too narrow to show everything on one row, the
//! content is packed across as many lines as needed so no badge clips.

use ratatui::text::Line;

use super::badges::badge_spans;
use super::status_metrics::metric_spans;
use crate::tui::app::state::App;

/// Width below which the status bar stacks across multiple rows.
pub use codetether_chat_layout::STACK_WIDTH_THRESHOLD;
/// Assemble the responsive status bar from native badge and metric providers.
pub fn build_status_lines(app: &App, session_label: &str, width: u16) -> Vec<Line<'static>> {
    let badges = badge_spans(app, session_label);
    let metrics = metric_spans(app);
    codetether_chat_status::build_status_lines(badges, metrics, width)
}

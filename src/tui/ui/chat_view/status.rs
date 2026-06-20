//! Bottom status-line span assembly.
//!
//! [`build_status_lines`] collects keybinding hints, session label,
//! auto-apply badge, bus status, latency, token usage, and status
//! text into one or more [`Line`]s for the status bar. When the
//! terminal is too narrow to show everything on one row, the
//! content is packed across as many lines as needed so no badge clips.

use ratatui::text::{Line, Span};

use super::badges::badge_spans;
use super::status_hints::keybinding_spans;
use super::status_metrics::metric_spans;
use crate::tui::app::state::App;

/// Width below which the status bar stacks across multiple rows.
pub const STACK_WIDTH_THRESHOLD: u16 = 180;

/// Assemble the status bar as one or more lines.
///
/// Returns a single line when `width >= STACK_WIDTH_THRESHOLD`,
/// otherwise width-packs hints, badges, and metrics across multiple
/// rows so every badge stays visible without manual zoom-out.
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
    let badges = badge_spans(app, session_label);
    let metrics = metric_spans(app);

    if width >= STACK_WIDTH_THRESHOLD {
        let mut combined = keybinding_spans();
        combined.push(Span::raw(" | "));
        combined.extend(badges);
        combined.push(Span::raw(" | "));
        combined.extend(metrics);
        vec![Line::from(combined)]
    } else {
        super::status_pack::pack_stacked(
            super::compact_hints::compact_keybinding_spans(),
            badges,
            metrics,
            width,
        )
    }
}

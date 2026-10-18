//! Live "processing" badge: animated neon spinner + active-agent count.
//!
//! The spinner glyph and color cycle from wall-clock time. The event loop
//! only redraws while `app.state.processing` is true (see
//! `event_loop::dirty`), so the animation runs during a turn and freezes
//! at 0% idle CPU once the turn completes.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use codetether_tui_style::spinner::{current_spinner_frame, spinner_color};

/// Render an active-turn badge and optional concurrent task count.
///
/// # Arguments
/// * `processing` - Whether a turn is running.
/// * `task_count` - Lazy count, queried only for active turns.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::processing_badge::processing_badge(true, || 0);
/// ```
pub fn processing_badge(
    processing: bool,
    task_count: impl FnOnce() -> usize,
) -> Option<Span<'static>> {
    if !processing {
        return None;
    }
    let tasks = task_count();
    let label = if tasks > 0 {
        format!(" {} working ({tasks} tasks) ", current_spinner_frame())
    } else {
        format!(" {} working ", current_spinner_frame())
    };
    Some(Span::styled(
        label,
        Style::default()
            .fg(Color::Black)
            .bg(spinner_color())
            .add_modifier(Modifier::BOLD),
    ))
}

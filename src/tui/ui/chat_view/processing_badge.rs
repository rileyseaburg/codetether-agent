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

use crate::tui::app::state::App;

use super::spinner::{current_spinner_frame, spinner_color};

/// Build the processing badge span when a turn is in flight.
///
/// Returns `None` when idle. While processing, shows an animated neon
/// spinner. A task count is appended only for genuine concurrent work.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::processing_badge::processing_badge;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let _maybe = processing_badge(app);
/// # }
/// ```
pub fn processing_badge(app: &App) -> Option<Span<'static>> {
    if !app.state.processing {
        return None;
    }
    let tasks = app.state.worker_task_queue.len()
        + usize::from(app.state.active_remote_task.is_some())
        + app.state.active_tasks.count();
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

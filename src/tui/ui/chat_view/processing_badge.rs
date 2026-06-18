//! Live "processing" badge: a gated spinner plus active-agent count.
//!
//! The spinner glyph is derived from wall-clock time, but the surrounding
//! event loop only redraws while `app.state.processing` is true (see
//! `event_loop::dirty`), so the animation runs during a turn and freezes
//! at 0% idle CPU once the turn completes.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;
use crate::tui::ui::chat_view::spinner::current_spinner_frame;

/// Build the processing badge span when a turn is in flight.
///
/// Returns `None` when idle so the status bar stays static and no redraw
/// is triggered. While processing it shows the animated spinner and, when
/// present, the number of registered worker agents.
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
    let agents = app.state.worker_bridge_registered_agents.len();
    let label = if agents > 0 {
        format!(" {} working ({agents} agents) ", current_spinner_frame())
    } else {
        format!(" {} working ", current_spinner_frame())
    };
    Some(Span::styled(
        label,
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

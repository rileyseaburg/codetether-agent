//! Pulsing `⚡ YOLO` badge shown when full-auto mode is active.

use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;

use super::spinner::spinner_color;

/// Returns a pulsing neon `⚡ YOLO` badge when `--yolo` / auto-apply is on.
///
/// The badge color cycles through neon hues so it catches the eye immediately.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::yolo_badge::yolo_badge;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let _badge = yolo_badge(app);
/// # }
/// ```
pub fn yolo_badge(app: &App) -> Option<Span<'static>> {
    if !app.state.auto_apply_edits {
        return None;
    }
    Some(Span::styled(
        " ⚡ YOLO ",
        Style::default()
            .fg(spinner_color())
            .add_modifier(Modifier::BOLD),
    ))
}

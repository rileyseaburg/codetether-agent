//! Pulsing `⚡ YOLO` badge shown when full-auto mode is active.
//!
//! On truecolor terminals the badge uses an inverted neon block:
//! black text on a spinning neon background — unmissable.
//! On 8-color terminals it stays bold yellow text.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;
use crate::tui::ui::chat_view::spinner::spinner_color;
use crate::tui::ui::gradient::rgb_supported;

/// Returns a pulsing neon `⚡ YOLO` badge when `--yolo` / auto-apply is on.
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
    let style = if rgb_supported() {
        Style::default()
            .fg(Color::Black)
            .bg(spinner_color())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    };
    Some(Span::styled(" ⚡ YOLO ", style))
}

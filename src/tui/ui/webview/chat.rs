//! Bind transcript scroll state and composer cursor placement.
use crate::tui::{app::state::App, models::InputMode};
/// Shared minimum-size predicate.
pub use codetether_webview::chat::terminal_too_small;
use ratatui::{Frame, layout::Rect, text::Line};
pub fn render_webview_chat_center(f: &mut Frame, app: &App, area: Rect, lines: &[Line<'static>]) {
    codetether_webview::chat::render_center(f, app.state.chat_viewport.chat_scroll, area, lines);
}
pub fn render_webview_input(f: &mut Frame, app: &App, area: Rect) {
    codetether_webview::chat::render_input(
        f,
        &app.state.composer.input,
        app.state.composer.mode == InputMode::Command,
        area,
    );
    crate::tui::ui::chat_view::cursor::place_cursor(f, app, area);
}

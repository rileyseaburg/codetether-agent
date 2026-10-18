//! Supply the token summary and native status values to shared rendering.
use crate::tui::{
    app::state::App, theme::Theme, theme_utils::validate_theme, token_display::TokenDisplay,
};
use ratatui::{Frame, layout::Rect};
pub fn render_webview_status(f: &mut Frame, app: &App, area: Rect) {
    let token_display = TokenDisplay::new();
    let validated_theme = validate_theme(&Theme::default());
    let status_line = token_display.create_status_bar(&validated_theme);
    let prefix = super::status_prefix::build(app);
    codetether_webview_bars::status::render(
        f,
        status_line,
        prefix,
        app.state.last_completion_model.as_deref(),
        area,
    );
}
/// Rows needed by the status bar at the available width.
pub use codetether_chat_layout::webview::status_height;
/// Fallback message shown when the terminal is too small for Webview.
pub use codetether_webview_bars::status::render_too_small;

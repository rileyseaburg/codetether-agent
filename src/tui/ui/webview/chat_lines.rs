//! Chat-line construction for the webview center panel.
//!
//! The webview previously read the message-line cache directly via
//! `get_or_build_message_lines`, which returns `None` on a cold cache —
//! leaving the center panel permanently blank. This module builds lines
//! through the same cached pipeline as the classic chat view.

use ratatui::Frame;
use ratatui::layout::Rect;

use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::chat_view::lines::build_chat_lines;

/// Build (or fetch cached) chat lines for `center_area`, render the
/// center panel, and restore the cache.
pub(super) fn render_center(f: &mut Frame, app: &mut App, center_area: Rect) {
    let max_w = center_area.width.saturating_sub(4) as usize;
    let palette = ColorPalette::marketing();
    let formatter = MessageFormatter::new(max_w);
    let drawn = build_chat_lines(app, max_w, max_w, &formatter, &palette);
    let vis = center_area.height.saturating_sub(2) as usize;
    app.state
        .set_chat_max_scroll(drawn.as_slice().len().saturating_sub(vis));
    super::chat::render_webview_chat_center(f, app, center_area, drawn.as_slice());
    drawn.restore(app, max_w);
}

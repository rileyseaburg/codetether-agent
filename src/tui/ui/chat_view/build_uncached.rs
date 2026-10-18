//! Apply application scroll limits after shared transcript construction.
use crate::tui::{
    app::state::App, color_palette::ColorPalette, message_formatter::MessageFormatter,
};
use ratatui::text::Line;
pub(super) fn build_uncached(
    app: &mut App,
    separator_width: usize,
    panel_width: usize,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> Vec<Line<'static>> {
    let rendered = codetether_tool_panel::build_transcript(
        &app.state.messages,
        separator_width,
        panel_width,
        app.state.tool_viewport.tool_preview_scroll,
        app.state.pending_tool_snapshot(),
        formatter,
        palette,
        || crate::tui::app::state::approval_queue::active().map(|item| item.tool),
    );
    app.state.set_tool_preview_max_scroll(rendered.max_scroll);
    rendered.lines
}

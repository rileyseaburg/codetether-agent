//! Slow-path: build the full chat-line buffer when the cache is cold.

use ratatui::text::Line;

use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::tool_panel::build_render_entries;

use super::empty::push_empty_placeholder;
use super::entries::append_entries;

pub(super) fn build_uncached(
    app: &mut App,
    separator_width: usize,
    panel_width: usize,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let entries = build_render_entries(&app.state.messages);
    let pending = app.state.pending_tool_snapshot();
    if entries.is_empty() {
        push_empty_placeholder(&mut lines);
        app.state.set_tool_preview_max_scroll(0);
    } else {
        let result = append_entries(
            &mut lines,
            &entries,
            separator_width,
            panel_width,
            app.state.tool_preview_scroll,
            pending,
            formatter,
            palette,
        );
        app.state
            .set_tool_preview_max_scroll(result.tool_preview_max_scroll);
    }
    lines
}

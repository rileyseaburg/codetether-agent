//! Render-line construction for the active swarm worker transcript.

use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::tool_panel::build_render_entries;

use super::drawn_lines::DrawnLines;
use super::entries::append_entries;

pub(super) fn build(
    app: &mut App,
    content_width: usize,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> Option<DrawnLines> {
    let messages = super::swarm_messages::active(&app.state)?;
    let entries = build_render_entries(&messages);
    let mut lines = Vec::new();
    let result = append_entries(
        &mut lines,
        &entries,
        content_width.saturating_sub(2).min(60),
        content_width.saturating_sub(4),
        0,
        None,
        formatter,
        palette,
    );
    app.state
        .set_tool_preview_max_scroll(result.tool_preview_max_scroll);
    Some(DrawnLines::from_rebuild(lines))
}

#[cfg(test)]
#[path = "swarm_lines_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "real_session_benchmark.rs"]
mod real_session_benchmark;

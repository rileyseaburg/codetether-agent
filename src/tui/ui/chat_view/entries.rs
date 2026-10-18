//! Application approval lookup for shared entry assembly.
use super::entry_result::EntryAppendResult;
use crate::tui::{
    color_palette::ColorPalette,
    message_formatter::MessageFormatter,
    ui::tool_panel::{PendingToolSnapshot, RenderEntry},
};
use ratatui::text::Line;
/// Assemble entries with the current pending approval state.
#[allow(clippy::too_many_arguments)]
pub fn append_entries(
    lines: &mut Vec<Line<'static>>,
    entries: &[RenderEntry<'_>],
    separator_width: usize,
    panel_width: usize,
    tool_preview_scroll: usize,
    pending: PendingToolSnapshot<'_>,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> EntryAppendResult {
    codetether_tool_panel::append_entries(
        lines,
        entries,
        separator_width,
        panel_width,
        tool_preview_scroll,
        pending,
        formatter,
        palette,
        || crate::tui::app::state::approval_queue::active().map(|item| item.tool),
    )
}

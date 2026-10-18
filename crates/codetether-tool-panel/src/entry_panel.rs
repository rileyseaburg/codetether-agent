//! Append one entry's tool panel before its chat message.
use crate::{PendingToolSnapshot, RenderEntry, build_tool_activity_panel};
use ratatui::text::Line;
#[allow(clippy::too_many_arguments)]
pub(super) fn append_panel(
    lines: &mut Vec<Line<'static>>,
    entry: &RenderEntry<'_>,
    tool_preview_scroll: usize,
    panel_width: usize,
    entry_pending: PendingToolSnapshot<'_>,
    tool_preview_max_scroll: &mut usize,
    approval_tool: &mut impl FnMut() -> Option<String>,
) {
    if !entry.tool_activity.is_empty() || entry_pending.is_some() {
        let panel = build_tool_activity_panel(
            &entry.tool_activity,
            tool_preview_scroll,
            panel_width,
            entry_pending,
            approval_tool,
        );
        *tool_preview_max_scroll = (*tool_preview_max_scroll).max(panel.max_scroll);
        lines.extend(panel.lines);
        if entry.message.is_some() {
            lines.push(Line::from(""));
        }
    }
}

//! Per-message rendering loop (tool panels, chat messages).

use ratatui::text::Line;

use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::tool_panel::{
    PendingToolSnapshot, RenderEntry, build_tool_activity_panel, render_chat_message,
};

use super::entry_result::EntryAppendResult;
use super::separator::push_separator;

/// Render separators, tool panels, and chat messages.
/// Returns [`EntryAppendResult`] tracking deepest tool-panel scroll.
///
/// `pending` is appended to the LAST entry's tool panel as a running indicator.
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
    let mut tool_preview_max_scroll = 0;
    let last_idx = entries.len().saturating_sub(1);
    for (idx, entry) in entries.iter().enumerate() {
        if idx > 0 {
            push_separator(lines, entry, separator_width);
        }
        let entry_pending = if idx == last_idx { pending } else { None };
        if !entry.tool_activity.is_empty() || entry_pending.is_some() {
            let panel = build_tool_activity_panel(
                &entry.tool_activity,
                tool_preview_scroll,
                panel_width,
                entry_pending,
            );
            tool_preview_max_scroll = tool_preview_max_scroll.max(panel.max_scroll);
            lines.extend(panel.lines);
            if entry.message.is_some() {
                lines.push(Line::from(""));
            }
        }
        if let Some(message) = entry.message {
            render_chat_message(lines, message, formatter, palette);
            lines.push(Line::from(""));
        }
    }
    EntryAppendResult {
        tool_preview_max_scroll,
    }
}

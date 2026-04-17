//! Per-message rendering loop (tool panels, chat messages).

use ratatui::text::Line;

use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::tool_panel::{
    RenderEntry, build_tool_activity_panel, render_chat_message,
};

use super::entry_result::EntryAppendResult;
use super::separator::push_separator;

/// Render separators, tool panels, and chat messages.
/// Returns [`EntryAppendResult`] tracking deepest tool-panel scroll.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::entries::append_entries;
/// # fn d(l:&mut Vec<ratatui::text::Line>,e:&[codetether_agent::tui::ui::tool_panel::RenderEntry],f:&codetether_agent::tui::message_formatter::MessageFormatter,p:&codetether_agent::tui::color_palette::ColorPalette){ let r=append_entries(l,e,40,36,0,f,p); }
/// ```
pub fn append_entries(
    lines: &mut Vec<Line<'static>>,
    entries: &[RenderEntry<'_>],
    separator_width: usize,
    panel_width: usize,
    tool_preview_scroll: usize,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> EntryAppendResult {
    let mut tool_preview_max_scroll = 0;
    for (idx, entry) in entries.iter().enumerate() {
        if idx > 0 {
            push_separator(lines, entry, separator_width);
        }
        if !entry.tool_activity.is_empty() {
            let panel =
                build_tool_activity_panel(&entry.tool_activity, tool_preview_scroll, panel_width);
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

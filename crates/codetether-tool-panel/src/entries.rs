//! Per-message rendering loop (tool panels, chat messages).

use ratatui::text::Line;

use crate::{PendingToolSnapshot, RenderEntry, render_chat_message};
use codetether_message_formatter::MessageFormatter;
use codetether_theme::ColorPalette;

use super::entry_result::EntryAppendResult;
use super::separator::push_separator;

/// Render separators, tool panels, and chat messages.
/// Returns [`EntryAppendResult`] tracking deepest tool-panel scroll.
///
/// `pending` is appended to the LAST entry's tool panel as a running indicator.
///
/// # Arguments
/// * `lines` - Destination transcript rows.
/// * `entries` - Grouped chat/tool entries.
/// * `separator_width` - Separator width.
/// * `panel_width` - Tool-panel width.
/// * `tool_preview_scroll` - Tool-panel scroll offset.
/// * `pending` - Pending tool appended to the final entry.
/// * `formatter` - Content formatter.
/// * `palette` - Message colors.
/// * `approval_tool` - Lazy active approval name lookup.
///
/// # Returns
/// The maximum tool-panel scroll offset seen while rendering.
///
/// # Examples
/// ```
/// let mut lines = vec![];
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// let palette = codetether_theme::ColorPalette::marketing();
/// let result = codetether_tool_panel::append_entries(&mut lines, &[], 60, 80, 0, None, &formatter, &palette, || None);
/// assert_eq!(result.tool_preview_max_scroll, 0);
/// ```
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
    mut approval_tool: impl FnMut() -> Option<String>,
) -> EntryAppendResult {
    let mut tool_preview_max_scroll = 0;
    let last_idx = entries.len().saturating_sub(1);
    for (idx, entry) in entries.iter().enumerate() {
        if idx > 0 {
            push_separator(lines, entry, separator_width);
        }
        let entry_pending = if idx == last_idx { pending } else { None };
        super::entry_panel::append_panel(
            lines,
            entry,
            tool_preview_scroll,
            panel_width,
            entry_pending,
            &mut tool_preview_max_scroll,
            &mut approval_tool,
        );
        if let Some(message) = entry.message {
            render_chat_message(lines, message, formatter, palette);
            lines.push(Line::from(""));
        }
    }
    EntryAppendResult {
        tool_preview_max_scroll,
    }
}

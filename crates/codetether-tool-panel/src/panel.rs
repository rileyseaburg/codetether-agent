//! Build rows for a compact tool-activity panel.
use super::{
    ToolPanelRender, item::render_tool_activity_item, panel_chrome::empty_body_lines,
    pending_spinner::append_pending_tool,
};
use codetether_chat_model::ChatMessage;
use std::time::Instant;
/// Snapshot of a pending tool name and its start time.
pub type PendingToolSnapshot<'a> = Option<(&'a str, Instant)>;

///
/// # Arguments
/// * `messages` - Tool activity in chronological order.
/// * `scroll_offset` - Requested panel scroll position.
/// * `width` - Available display width.
/// * `pending` - Optional live tool name and start time.
/// * `approval_tool` - Lazy active approval tool-name lookup.
///
/// # Returns
/// Formatted rows and their maximum scroll offset.
///
/// # Examples
/// ```
/// let panel = codetether_tool_panel::build_tool_activity_panel(&[], 0, 80, None, || None);
/// assert!(!panel.lines.is_empty());
/// ```
pub fn build_tool_activity_panel(
    messages: &[&ChatMessage],
    scroll_offset: usize,
    width: usize,
    pending: PendingToolSnapshot<'_>,
    approval_tool: impl FnOnce() -> Option<String>,
) -> ToolPanelRender {
    let header_width = width.max(24);
    let preview_width = header_width.saturating_sub(10).max(24);
    let mut body_lines = Vec::new();

    for message in messages {
        render_tool_activity_item(&mut body_lines, message, preview_width);
    }
    if let Some((name, started_at)) = pending {
        append_pending_tool(&mut body_lines, name, started_at, None, approval_tool);
    }
    if body_lines.is_empty() {
        body_lines = empty_body_lines();
    }

    super::panel_render::finish(messages, body_lines, scroll_offset, header_width)
}

//! Application approval adapter and shared tool-panel exports.
pub use codetether_tool_panel::{
    PendingToolSnapshot, RenderEntry, TOOL_PANEL_VISIBLE_LINES, ToolPanelRender, arg_preview,
    build_render_entries, icons, is_tool_activity, render_chat_message, separator_pattern,
};
/// Build the activity panel using the current live approval display state.
///
/// # Arguments
/// * `messages` - Tool activity in chronological order.
/// * `scroll_offset` - Requested panel scroll position.
/// * `width` - Available terminal width.
/// * `pending` - Optional in-flight tool name and start time.
///
/// # Returns
/// Styled tool rows and the maximum scroll offset.
///
/// # Examples
/// ```
/// let panel = codetether_agent::tui::ui::tool_panel::build_tool_activity_panel(&[], 0, 80, None);
/// assert!(!panel.lines.is_empty());
/// ```
pub fn build_tool_activity_panel(
    messages: &[&crate::tui::chat::message::ChatMessage],
    scroll_offset: usize,
    width: usize,
    pending: PendingToolSnapshot<'_>,
) -> ToolPanelRender {
    codetether_tool_panel::build_tool_activity_panel(
        messages,
        scroll_offset,
        width,
        pending,
        || crate::tui::app::state::approval_queue::active().map(|item| item.tool),
    )
}

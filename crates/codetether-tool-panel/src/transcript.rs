//! Cold transcript construction from shared message data.
mod detail;
mod viewport;
use crate::{PendingToolSnapshot, ToolPanelRender, append_entries, build_render_entries};
use codetether_chat_model::ChatMessage;
use codetether_message_formatter::MessageFormatter;
use codetether_theme::ColorPalette;
pub use viewport::TranscriptViewport;
/// Render a transcript and report the required tool-panel scroll limit.
///
/// # Arguments
/// * `messages` - Chronological chat messages.
/// * `separator_width` - Width of inter-entry separators.
/// * `panel_width` - Tool panel width.
/// * `tool_preview_scroll` - Requested tool scroll offset.
/// * `pending` - Optional pending tool snapshot.
/// * `formatter` - Content formatter.
/// * `palette` - Message colors.
/// * `approval_tool` - Lazy active approval tool-name lookup.
///
/// # Returns
/// Rendered lines and the maximum tool scroll offset.
///
/// # Examples
/// ```
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// let palette = codetether_theme::ColorPalette::marketing();
/// let rendered = codetether_tool_panel::build_transcript(&[], 60, 80, 0, None, &formatter, &palette, || None);
/// assert!(!rendered.lines.is_empty());
/// ```
#[allow(clippy::too_many_arguments)]
pub fn build_transcript(
    messages: &[ChatMessage],
    separator_width: usize,
    panel_width: usize,
    tool_preview_scroll: usize,
    pending: PendingToolSnapshot<'_>,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
    approval_tool: impl FnMut() -> Option<String>,
) -> ToolPanelRender {
    let mut lines = Vec::new();
    let entries = build_render_entries(messages);
    let max_scroll = if entries.is_empty() {
        codetether_tui_style::banner::push_welcome_banner(&mut lines);
        0
    } else {
        append_entries(
            &mut lines,
            &entries,
            separator_width,
            panel_width,
            tool_preview_scroll,
            pending,
            formatter,
            palette,
            approval_tool,
        )
        .tool_preview_max_scroll
    };
    ToolPanelRender { lines, max_scroll }
}

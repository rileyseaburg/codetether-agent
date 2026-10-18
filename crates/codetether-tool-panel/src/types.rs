//! Grouped transcript entries and rendered panel output.
use codetether_chat_model::ChatMessage;

/// A transcript entry carrying preceding tool activity and an optional message.
///
/// # Examples
/// ```
/// let entry = codetether_tool_panel::RenderEntry::default();
/// assert!(entry.message.is_none());
/// ```
#[derive(Default)]
pub struct RenderEntry<'a> {
    /// Tool messages grouped before this chat message.
    pub tool_activity: Vec<&'a ChatMessage>,
    /// Chat message following the group, when present.
    pub message: Option<&'a ChatMessage>,
}

/// Rendered activity rows and maximum scroll offset.
///
/// # Examples
/// ```
/// let panel = codetether_tool_panel::ToolPanelRender { lines: vec![], max_scroll: 0 };
/// assert!(panel.lines.is_empty());
/// ```
pub struct ToolPanelRender {
    /// Fully formatted terminal rows.
    pub lines: Vec<ratatui::text::Line<'static>>,
    /// Largest valid scroll offset.
    pub max_scroll: usize,
}

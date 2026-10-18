//! Tool preview scroll position and content bounds.
//! The shared state owns viewport positions without application events.

/// Mutable tool-preview position using TOOL_PREVIEW_FOLLOW as its sentinel.
///
/// # Examples
/// ```
/// let state = codetether_tool_panel::ToolPreviewScrollState::default();
/// assert_eq!(state.tool_preview_scroll, 0);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ToolPreviewScrollState {
    /// Current position, or TOOL_PREVIEW_FOLLOW to follow the latest activity.
    pub tool_preview_scroll: usize,
    /// Last measured maximum scroll position.
    pub tool_preview_last_max_scroll: usize,
}

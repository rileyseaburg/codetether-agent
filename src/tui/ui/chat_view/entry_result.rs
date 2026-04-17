//! Result type returned by [`super::entries::append_entries`].

/// Result of the entry-append pass.
///
/// Carries the maximum scroll offset across all tool activity panels
/// so the outer layer can clamp the tool-preview scroll.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::entry_result::EntryAppendResult;
/// let result = EntryAppendResult { tool_preview_max_scroll: 42 };
/// assert_eq!(result.tool_preview_max_scroll, 42);
/// ```
pub struct EntryAppendResult {
    /// Largest scroll offset found in tool activity panels.
    pub tool_preview_max_scroll: usize,
}

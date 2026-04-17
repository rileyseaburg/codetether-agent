//! Layout chunks struct.

/// Rectangular chunks produced by [`super::layout_compute::compute_chat_chunks`].
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::layout_chunks::ChatChunks;
/// let c = ChatChunks {
///     messages: ratatui::layout::Rect::new(0, 0, 80, 20),
///     input: ratatui::layout::Rect::new(0, 20, 80, 3),
///     suggestions: None,
///     status: ratatui::layout::Rect::new(0, 23, 80, 1),
/// };
/// assert!(c.suggestions.is_none());
/// ```
pub struct ChatChunks {
    pub messages: ratatui::layout::Rect,
    pub input: ratatui::layout::Rect,
    pub suggestions: Option<ratatui::layout::Rect>,
    pub status: ratatui::layout::Rect,
}

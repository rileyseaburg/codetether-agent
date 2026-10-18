//! Layout chunks struct.

/// Rectangular chunks produced by [`super::layout_compute::compute_chat_chunks`].
///
/// # Examples
///
/// ```rust
/// use codetether_chat_layout::ChatChunks;
/// let c = ChatChunks {
///     agent_bar: ratatui::layout::Rect::new(0, 0, 80, 0),
///     messages: ratatui::layout::Rect::new(0, 0, 80, 20),
///     input: ratatui::layout::Rect::new(0, 20, 80, 3),
///     suggestions: None,
///     status: ratatui::layout::Rect::new(0, 23, 80, 1),
/// };
/// assert!(c.suggestions.is_none());
/// ```
pub struct ChatChunks {
    /// Header row listing active agents (height 0 when none are spawned).
    pub agent_bar: ratatui::layout::Rect,
    /// Pre-wrapped conversation lines.
    pub messages: ratatui::layout::Rect,
    /// Composer panel.
    pub input: ratatui::layout::Rect,
    /// Optional completion list.
    pub suggestions: Option<ratatui::layout::Rect>,
    /// Responsive bottom status strip.
    pub status: ratatui::layout::Rect,
}

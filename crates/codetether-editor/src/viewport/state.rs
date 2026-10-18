//! Borrowed editor content and mutable viewport offsets.
use crate::FileBuffer;
use ratatui::layout::Rect;

/// Editor data needed to draw one frame and follow its cursor.
/// # Examples
/// ```
/// let (mut scroll, mut hscroll) = (0, 0);
/// let mut area = ratatui::layout::Rect::default();
/// let view = codetether_editor::viewport::View {
///     buffer: None, scroll: &mut scroll, hscroll: &mut hscroll,
///     lsp_area: &mut area, hover: None, goal_active: false, status: "Ready",
/// };
/// assert!(view.buffer.is_none());
/// ```
pub struct View<'a> {
    /// Current document, if open.
    pub buffer: Option<&'a FileBuffer>,
    /// Vertical scroll offset updated to follow the cursor.
    pub scroll: &'a mut usize,
    /// Horizontal scroll offset updated to follow the cursor.
    pub hscroll: &'a mut usize,
    /// Document rectangle used by language-server interactions.
    pub lsp_area: &'a mut Rect,
    /// Current language-server hover text.
    pub hover: Option<&'a str>,
    /// Whether the editor is displaying a goal draft.
    pub goal_active: bool,
    /// Status or conflict text displayed beneath a goal draft.
    pub status: &'a str,
}

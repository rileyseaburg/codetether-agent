//! Host-manager state, hover text and editor hit-test bounds.
use ratatui::layout::Rect;
use std::sync::Arc;

/// LSP-related editor state attached to the app.

/// # Examples
/// ```
/// let mut state = codetether_editor_lsp::EditorLspState::<()>::default();
/// state.hover = Some("documentation".into());
/// assert!(state.clone().has_hover());
/// state.clear_hover();
/// assert!(!state.has_hover());
/// ```
pub struct EditorLspState<M> {
    /// Lazily created LSP manager shared across editor requests.
    pub manager: Option<Arc<M>>,
    /// Rendered hover/JSDoc markdown to show as a popup, if any.
    pub hover: Option<String>,
    /// Editor draw area from the last frame, for click hit-testing.
    pub area: Rect,
}

impl<M> EditorLspState<M> {
    /// Whether a hover popup is currently visible.
    pub fn has_hover(&self) -> bool {
        self.hover.is_some()
    }

    /// Dismisses any visible hover popup.
    pub fn clear_hover(&mut self) {
        self.hover = None;
    }
}

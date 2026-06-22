//! Editor-side LSP state: a shared [`LspManager`] and a transient hover popup.
//!
//! Holds only data, no I/O. The async key handler fills [`EditorLspState::hover`]
//! after a `textDocument/hover` request and clears it when the user dismisses it.

use std::sync::Arc;

use ratatui::layout::Rect;

use crate::lsp::LspManager;

/// LSP-related editor state attached to the app.
#[derive(Clone, Default)]
pub struct EditorLspState {
    /// Lazily created LSP manager shared across editor requests.
    pub manager: Option<Arc<LspManager>>,
    /// Rendered hover/JSDoc markdown to show as a popup, if any.
    pub hover: Option<String>,
    /// Editor draw area from the last frame, for click hit-testing.
    pub area: Rect,
}

impl EditorLspState {
    /// Whether a hover popup is currently visible.
    pub fn has_hover(&self) -> bool {
        self.hover.is_some()
    }

    /// Dismisses any visible hover popup.
    pub fn clear_hover(&mut self) {
        self.hover = None;
    }
}

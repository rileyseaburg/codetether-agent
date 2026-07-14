mod lifecycle;
mod navigation;

use ratatui::widgets::ListState;
use std::path::PathBuf;

use super::SymbolEntry;

/// Determines whether selection navigates to code or attaches chat context.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SymbolSearchMode {
    /// Preserve the original Ctrl+T navigation behavior.
    #[default]
    Navigate,
    /// Replace the `@` at the recorded character position.
    Mention { start: usize },
}

/// Interactive workspace-symbol picker state.
#[derive(Debug, Default)]
pub struct SymbolSearchState {
    /// Whether the popup is visible and consumes input.
    pub active: bool,
    /// Current workspace-symbol query.
    pub query: String,
    /// Ranked language-server results.
    pub results: Vec<SymbolEntry>,
    /// Highlighted result index.
    pub selected: usize,
    /// Whether a language-server request is running.
    pub loading: bool,
    /// Last search error, if any.
    pub error: Option<String>,
    /// Ratatui selection state.
    pub list_state: ListState,
    pub(crate) language_files: Vec<PathBuf>,
    /// Action performed when a result is confirmed.
    pub mode: SymbolSearchMode,
}

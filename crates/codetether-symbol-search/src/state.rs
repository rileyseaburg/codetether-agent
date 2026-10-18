//! Symbol picker state.
mod lifecycle;
mod navigation;

use ratatui::widgets::ListState;
use std::path::PathBuf;

use super::SymbolEntry;

/// Determines whether selection navigates to code or attaches chat context.
///
/// # Examples
/// ```
/// use codetether_symbol_search::SymbolSearchMode;
/// assert_eq!(match SymbolSearchMode::default() {
///     SymbolSearchMode::Navigate => "navigate",
///     SymbolSearchMode::Mention { .. } => "mention",
/// }, "navigate");
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SymbolSearchMode {
    /// Preserve the original Ctrl+T navigation behavior.
    #[default]
    Navigate,
    /// Replace the `@` at the recorded character position.
    Mention {
        /// Character offset of the mention marker in the composer.
        start: usize,
    },
}

/// Interactive workspace-symbol picker state.
///
/// # Examples
/// ```
/// let mut picker = codetether_symbol_search::SymbolSearchState::new();
/// picker.open_mention(2);
/// assert!(picker.active);
/// ```
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
    pub(super) language_files: Vec<PathBuf>,
    /// Action performed when a result is confirmed.
    pub mode: SymbolSearchMode,
}

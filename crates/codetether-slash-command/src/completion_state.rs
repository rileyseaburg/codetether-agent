//! Completion candidates and their selected index.
//! Use CompletionState to manage command candidates without UI dependencies.

/// Mutable candidates and selection for one command input.
///
/// # Examples
/// ```
/// let state = codetether_slash_command::CompletionState::default();
/// assert!(state.slash_suggestions.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct CompletionState {
    /// Ordered matches from the command catalog.
    pub slash_suggestions: Vec<String>,
    /// Selected candidate index, clamped when candidates refresh.
    pub selected_slash_suggestion: usize,
}

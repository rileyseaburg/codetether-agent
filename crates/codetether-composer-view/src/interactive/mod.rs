//! Composer edit, history and completion state transitions.
//!
//! Borrow native fields through [Composer] to edit input while keeping history,
//! completion, input mode and cursor updates in their existing order.
mod cursor;
mod edit;
mod history;
mod replace;
mod suggestions;
use crate::CommandHistory;
use codetether_slash_command::CompletionState;
use codetether_tui_models::InputMode;
pub use suggestions::{suggestions_navigable, suggestions_visible};
/// Borrowed composer fields updated together by input operations.
/// # Examples
/// ```
/// let (mut input, mut cursor, mut scroll) = (String::new(), 0, 0);
/// let mut mode = codetether_tui_models::InputMode::Normal;
/// let mut completion = codetether_slash_command::CompletionState::default();
/// let mut history = codetether_composer_view::CommandHistory::default();
/// let mut composer = codetether_composer_view::interactive::Composer {
///     input: &mut input, cursor: &mut cursor, scroll: &mut scroll,
///     mode: &mut mode, completion: &mut completion, history: &mut history,
/// };
/// composer.insert_char('é');
/// assert_eq!(*composer.cursor, 1);
/// ```
pub struct Composer<'a> {
    /// Current editable text.
    pub input: &'a mut String,
    /// Unicode-scalar cursor position.
    pub cursor: &'a mut usize,
    /// Horizontal scroll position.
    pub scroll: &'a mut usize,
    /// Current input mode.
    pub mode: &'a mut InputMode,
    /// Completion candidates and selection.
    pub completion: &'a mut CompletionState,
    /// Submitted input history and recall position.
    pub history: &'a mut CommandHistory,
}

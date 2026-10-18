//! Owned text, cursor, completion and history state for a chat composer.
use crate::{CommandHistory, interactive::Composer};
use codetether_slash_command::CompletionState;
use codetether_tui_models::InputMode;

/// Persistent input state used by the composer's editing workflow.
///
/// Frontends select when to submit the text. Editing through [`Self::borrow`]
/// keeps cursor, completion and history updates coordinated.
///
/// # Examples
/// ```
/// let mut state = codetether_composer_view::ComposerState::default();
/// state.borrow().insert_text("hello");
/// assert_eq!(state.composer.input, "hello");
/// assert_eq!(state.cursor, 5);
/// ```
pub struct ComposerState {
    /// Current editable prompt text.
    pub input: String,
    /// Cursor position measured in Unicode scalar values.
    pub cursor: usize,
    /// Horizontal input scroll offset.
    pub scroll: usize,
    /// Current keyboard input mode.
    pub mode: InputMode,
    /// Slash-command candidates and selection.
    pub completion: CompletionState,
    /// Submitted prompts and history recall position.
    pub history: CommandHistory,
}

impl Default for ComposerState {
    fn default() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            scroll: 0,
            mode: InputMode::Normal,
            completion: CompletionState::default(),
            history: CommandHistory::default(),
        }
    }
}

impl ComposerState {
    /// Borrow all fields for a coordinated edit or history operation.
    /// # Arguments
    /// Borrows this composer state exclusively for the returned view's lifetime.
    /// # Returns
    /// An editing view over the existing fields, without copying them.
    /// # Examples
    /// ```
    /// let mut state = codetether_composer_view::ComposerState::default();
    /// state.borrow().insert_char('é');
    /// assert_eq!(state.cursor, 1);
    /// ```
    pub fn borrow(&mut self) -> Composer<'_> {
        Composer {
            input: &mut self.input,
            cursor: &mut self.cursor,
            scroll: &mut self.scroll,
            mode: &mut self.mode,
            completion: &mut self.completion,
            history: &mut self.history,
        }
    }
}

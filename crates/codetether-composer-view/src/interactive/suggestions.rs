//! Suggestion visibility, refresh and application.
use super::Composer;
use codetether_slash_command::CompletionState;
use codetether_tui_models::{InputMode, ViewMode};
/// Whether the chat composer should show candidates or a command usage hint.
/// # Arguments
/// Receives active view, processing flag and input text.
/// # Returns
/// True for slash input in an idle chat view, including text after a space.
/// # Examples
/// ```
/// assert!(codetether_composer_view::interactive::suggestions_visible(&codetether_tui_models::ViewMode::Chat, false, "/help"));
/// ```
pub fn suggestions_visible(view: &ViewMode, processing: bool, input: &str) -> bool {
    *view == ViewMode::Chat && !processing && input.starts_with('/')
}
/// Whether visible suggestions contain selectable candidates.
/// # Arguments
/// Receives active view, processing flag, input text and completion state.
/// # Returns
/// True only when the visible candidate list is nonempty.
/// # Examples
/// ```
/// let completion = codetether_slash_command::CompletionState::default();
/// assert!(!codetether_composer_view::interactive::suggestions_navigable(&codetether_tui_models::ViewMode::Chat, false, "/", &completion));
/// ```
pub fn suggestions_navigable(
    view: &ViewMode,
    processing: bool,
    input: &str,
    completion: &CompletionState,
) -> bool {
    suggestions_visible(view, processing, input) && !completion.slash_suggestions.is_empty()
}
impl Composer<'_> {
    /// Refresh candidates without changing input or history.
    pub fn refresh(&mut self) {
        self.completion.refresh_slash_suggestions(self.input);
    }
    /// Apply the selected command, move the cursor to its end and refresh candidates.
    pub fn apply_selected_slash_suggestion(&mut self) -> bool {
        if let Some(selected) = self.completion.selected_slash_suggestion() {
            *self.input = selected.to_string();
            *self.cursor = crate::cursor_index::input_char_count(self.input);
            *self.mode = InputMode::Command;
            self.refresh();
            return true;
        }
        false
    }
}

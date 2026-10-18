//! Slash autocomplete suggestion methods.
//!
//! Refreshing, navigating, and applying slash command completions.

use crate::tui::models::InputMode;

impl super::AppState {
    pub fn refresh_slash_suggestions(&mut self) {
        self.completion.refresh_slash_suggestions(&self.input);
    }

    pub fn slash_suggestions_visible(&self) -> bool {
        // Stay visible past the first space so the suggestions panel can
        // swap in a usage hint for the typed command — see
        // [`AppState::current_slash_hint`].
        self.view_mode == crate::tui::models::ViewMode::Chat
            && !self.processing
            && self.input.starts_with('/')
    }

    /// True only when there is a non-empty prefix-match list the user can
    /// navigate with Tab / Up / Down / PageUp-Down. Distinct from
    /// [`AppState::slash_suggestions_visible`], which also returns true
    /// when the panel is showing a usage hint with no list to navigate.
    pub fn slash_suggestions_navigable(&self) -> bool {
        self.slash_suggestions_visible() && !self.completion.slash_suggestions.is_empty()
    }

    /// Usage hint for the currently-typed slash command, if any.
    ///
    /// Looks up the first whitespace-separated token of `input` in
    /// [`super::slash_hints`]. Used by the chat-view suggestions panel to
    /// render `<args>` documentation once the prefix-match list is empty.
    pub fn current_slash_hint(&self) -> Option<&'static str> {
        codetether_slash_command::current_slash_hint(&self.input)
    }

    pub fn select_prev_slash_suggestion(&mut self) {
        self.completion.select_prev_slash_suggestion();
    }

    pub fn select_next_slash_suggestion(&mut self) {
        self.completion.select_next_slash_suggestion();
    }

    pub fn selected_slash_suggestion(&self) -> Option<&str> {
        self.completion.selected_slash_suggestion()
    }

    pub fn apply_selected_slash_suggestion(&mut self) -> bool {
        if let Some(selected) = self.selected_slash_suggestion() {
            self.input = selected.to_string();
            self.input_cursor = self.input_char_count();
            self.input_mode = InputMode::Command;
            self.refresh_slash_suggestions();
            return true;
        }
        false
    }
}

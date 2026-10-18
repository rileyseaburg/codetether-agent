//! Slash autocomplete suggestion methods.
//!
//! Refreshing, navigating, and applying slash command completions.

impl super::AppState {
    pub fn refresh_slash_suggestions(&mut self) {
        self.completion.refresh_slash_suggestions(&self.input);
    }

    pub fn slash_suggestions_visible(&self) -> bool {
        codetether_composer_view::interactive::suggestions_visible(
            &self.view_mode,
            self.processing,
            &self.input,
        )
    }
    pub fn slash_suggestions_navigable(&self) -> bool {
        codetether_composer_view::interactive::suggestions_navigable(
            &self.view_mode,
            self.processing,
            &self.input,
            &self.completion,
        )
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
        self.composer().apply_selected_slash_suggestion()
    }
}

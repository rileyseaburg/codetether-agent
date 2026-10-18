//! Owned composer completion operations.
impl crate::ComposerState {
    /// Refresh slash-command candidates for the current input.
    pub fn refresh_slash_suggestions(&mut self) {
        self.completion.refresh_slash_suggestions(&self.input);
    }
    /// Return the usage hint for the current slash command.
    pub fn current_slash_hint(&self) -> Option<&'static str> {
        crate::state_binding::current_slash_hint(&self.input)
    }
    /// Select the preceding slash-command candidate.
    pub fn select_prev_slash_suggestion(&mut self) {
        self.completion.select_prev_slash_suggestion();
    }
    /// Select the following slash-command candidate.
    pub fn select_next_slash_suggestion(&mut self) {
        self.completion.select_next_slash_suggestion();
    }
    /// Borrow the selected slash-command candidate, if any.
    pub fn selected_slash_suggestion(&self) -> Option<&str> {
        self.completion.selected_slash_suggestion()
    }
    /// Apply the selected candidate and update cursor, mode and completion.
    pub fn apply_selected_slash_suggestion(&mut self) -> bool {
        self.borrow().apply_selected_slash_suggestion()
    }
}

//! Refreshing and clearing completion candidates.
//! Use CompletionState to manage command candidates without UI dependencies.

impl crate::CompletionState {
    /// Refresh matches and clamp selection for the current input.
    ///
    /// # Arguments
    /// * `input` — Current composer text.
    ///
    /// # Returns
    /// Updates the candidates and selection in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_slash_command::CompletionState::default();
    /// s.refresh_slash_suggestions("/hel");
    /// assert_eq!(s.selected_slash_suggestion(), Some("/help"));
    /// ```

    pub fn refresh_slash_suggestions(&mut self, input: &str) {
        if !input.starts_with('/') {
            self.slash_suggestions.clear();
            self.selected_slash_suggestion = 0;
            return;
        }
        self.slash_suggestions = crate::suggestions(input);
        if self.selected_slash_suggestion >= self.slash_suggestions.len() {
            self.selected_slash_suggestion = self.slash_suggestions.len().saturating_sub(1);
        }
    }
    /// Clear matches and reset selection.
    ///
    /// # Returns
    /// Updates the candidates and selection in place.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_slash_command::CompletionState::default();
    /// s.clear();
    /// assert!(s.slash_suggestions.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.slash_suggestions.clear();
        self.selected_slash_suggestion = 0;
    }
}

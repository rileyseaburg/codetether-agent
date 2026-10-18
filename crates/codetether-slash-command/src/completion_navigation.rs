//! Selecting and reading command candidates.
//! Use CompletionState to manage command candidates without UI dependencies.

impl crate::CompletionState {
    /// Select the preceding candidate without wrapping.
    ///
    /// # Returns
    /// Updates the selected index when candidates exist.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_slash_command::CompletionState::default();
    /// s.select_prev_slash_suggestion();
    /// assert_eq!(s.selected_slash_suggestion, 0);
    /// ```

    pub fn select_prev_slash_suggestion(&mut self) {
        if !self.slash_suggestions.is_empty() {
            self.selected_slash_suggestion = self.selected_slash_suggestion.saturating_sub(1);
        }
    }
    /// Select the following candidate without wrapping.
    ///
    /// # Returns
    /// Updates the selected index when candidates exist.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_slash_command::CompletionState::default();
    /// s.select_next_slash_suggestion();
    /// assert_eq!(s.selected_slash_suggestion, 0);
    /// ```

    pub fn select_next_slash_suggestion(&mut self) {
        if !self.slash_suggestions.is_empty() {
            self.selected_slash_suggestion =
                (self.selected_slash_suggestion + 1).min(self.slash_suggestions.len() - 1);
        }
    }
    /// Read the selected candidate.
    ///
    /// # Returns
    /// The selected command, or `None` if the index is outside the list.
    ///
    /// # Examples
    /// ```
    /// let mut s = codetether_slash_command::CompletionState::default();
    /// assert_eq!(s.selected_slash_suggestion(), None);
    /// ```

    pub fn selected_slash_suggestion(&self) -> Option<&str> {
        self.slash_suggestions
            .get(self.selected_slash_suggestion)
            .map(String::as_str)
    }
}

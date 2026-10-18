//! Owned composer visibility operations.
impl crate::ComposerState {
    /// Check whether idle chat input should show slash-command suggestions.
    /// # Arguments
    /// * `view_mode` — Current frontend view.
    /// * `processing` — Whether a request is running.
    /// # Returns
    /// Whether the existing visibility policy permits this interaction.
    /// # Examples
    /// ```
    /// let state = codetether_composer_view::ComposerState::default();
    /// assert!(!state.slash_suggestions_visible(&codetether_tui_models::ViewMode::Chat, false));
    /// ```
    pub fn slash_suggestions_visible(
        &self,
        view_mode: &codetether_tui_models::ViewMode,
        processing: bool,
    ) -> bool {
        crate::interactive::suggestions_visible(view_mode, processing, &self.input)
    }
    /// Check whether visible slash-command candidates can be selected.
    /// # Arguments
    /// * `view_mode` — Current frontend view.
    /// * `processing` — Whether a request is running.
    /// # Returns
    /// Whether the existing visibility policy permits this interaction.
    /// # Examples
    /// ```
    /// let state = codetether_composer_view::ComposerState::default();
    /// assert!(!state.slash_suggestions_navigable(&codetether_tui_models::ViewMode::Chat, false));
    /// ```
    pub fn slash_suggestions_navigable(
        &self,
        view_mode: &codetether_tui_models::ViewMode,
        processing: bool,
    ) -> bool {
        crate::interactive::suggestions_navigable(
            view_mode,
            processing,
            &self.input,
            &self.completion,
        )
    }
}

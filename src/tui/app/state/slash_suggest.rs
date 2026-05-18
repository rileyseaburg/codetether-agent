//! Slash autocomplete suggestion methods.
//!
//! Refreshing, navigating, and applying slash command completions.

use crate::tui::models::InputMode;

impl super::AppState {
    pub fn refresh_slash_suggestions(&mut self) {
        if !self.input.starts_with('/') {
            self.slash_suggestions.clear();
            self.selected_slash_suggestion = 0;
            return;
        }
        let raw_query = self.input.split_whitespace().next().unwrap_or("");
        let query = crate::tui::app::text::normalize_slash_command(raw_query).to_lowercase();
        let mut matches: Vec<&str> = super::slash_commands::SLASH_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&query))
            .copied()
            .collect();
        if matches.is_empty() && query.len() > 1 {
            let needle = &query[1..];
            matches = super::slash_commands::SLASH_COMMANDS
                .iter()
                .filter(|cmd| cmd.contains(needle))
                .copied()
                .collect();
        }
        if matches.is_empty() && query == "/" {
            matches = super::slash_commands::SLASH_COMMANDS
                .iter()
                .copied()
                .collect();
        }
        self.slash_suggestions = matches.into_iter().map(String::from).collect();
        if self.selected_slash_suggestion >= self.slash_suggestions.len() {
            self.selected_slash_suggestion = self.slash_suggestions.len().saturating_sub(1);
        }
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
        self.slash_suggestions_visible() && !self.slash_suggestions.is_empty()
    }

    /// Usage hint for the currently-typed slash command, if any.
    ///
    /// Looks up the first whitespace-separated token of `input` in
    /// [`super::slash_hints`]. Used by the chat-view suggestions panel to
    /// render `<args>` documentation once the prefix-match list is empty.
    pub fn current_slash_hint(&self) -> Option<&'static str> {
        let cmd = self.input.split_whitespace().next()?;
        super::slash_hints::usage_hint(cmd)
    }

    pub fn select_prev_slash_suggestion(&mut self) {
        if !self.slash_suggestions.is_empty() {
            self.selected_slash_suggestion = self.selected_slash_suggestion.saturating_sub(1);
        }
    }

    pub fn select_next_slash_suggestion(&mut self) {
        if !self.slash_suggestions.is_empty() {
            self.selected_slash_suggestion =
                (self.selected_slash_suggestion + 1).min(self.slash_suggestions.len() - 1);
        }
    }

    pub fn selected_slash_suggestion(&self) -> Option<&str> {
        self.slash_suggestions
            .get(self.selected_slash_suggestion)
            .map(String::as_str)
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

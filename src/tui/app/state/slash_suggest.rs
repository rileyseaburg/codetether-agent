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
        self.slash_suggestions = super::slash_commands::SLASH_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&query))
            .map(|cmd| (*cmd).to_string())
            .collect();
        if self.slash_suggestions.is_empty() && query == "/" {
            self.slash_suggestions = super::slash_commands::SLASH_COMMANDS
                .iter()
                .map(|cmd| (*cmd).to_string())
                .collect();
        }
        if self.selected_slash_suggestion >= self.slash_suggestions.len() {
            self.selected_slash_suggestion = self.slash_suggestions.len().saturating_sub(1);
        }
    }

    pub fn slash_suggestions_visible(&self) -> bool {
        self.view_mode == crate::tui::models::ViewMode::Chat
            && !self.processing
            && self.input.starts_with('/')
            && !self.input.trim().chars().any(char::is_whitespace)
            && !self.slash_suggestions.is_empty()
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

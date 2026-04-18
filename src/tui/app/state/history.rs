//! Command history navigation (↑/↓ recall).
//!
//! Browsing mode cycles through previous inputs; releasing returns to editing.

use crate::tui::models::InputMode;

impl super::AppState {
    pub fn push_history(&mut self, entry: String) {
        if !entry.trim().is_empty() {
            self.command_history.push(entry);
            self.history_index = None;
        }
    }

    pub fn history_prev(&mut self) -> bool {
        if self.command_history.is_empty() {
            return false;
        }
        let history_len = self.command_history.len();
        let new_index = match self.history_index {
            Some(index) if index > 0 => Some(index - 1),
            Some(index) => Some(index),
            None => Some(history_len - 1),
        };
        if let Some(index) = new_index {
            self.history_index = Some(index);
            self.input = self.command_history[index].clone();
            self.input_cursor = self.input_char_count();
            self.input_mode = if self.input.starts_with('/') {
                InputMode::Command
            } else {
                InputMode::Editing
            };
            self.refresh_slash_suggestions();
            return true;
        }
        false
    }

    pub fn history_next(&mut self) -> bool {
        if self.command_history.is_empty() {
            return false;
        }
        match self.history_index {
            Some(index) if index + 1 < self.command_history.len() => {
                let next = index + 1;
                self.history_index = Some(next);
                self.input = self.command_history[next].clone();
            }
            Some(_) => {
                self.history_index = None;
                self.input.clear();
            }
            None => return false,
        }
        self.input_cursor = self.input_char_count();
        self.input_mode = if self.input.is_empty() {
            InputMode::Normal
        } else if self.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        self.refresh_slash_suggestions();
        true
    }
}

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;

use crate::provider::ProviderRegistry;
use crate::session::SessionSummary;
use crate::tui::bus_log::BusLogState;
use crate::tui::chat::message::ChatMessage;
use crate::tui::help::HelpScrollState;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::RalphViewState;
use crate::tui::swarm_view::SwarmViewState;
use crate::tui::symbol_search::SymbolSearchState;
use crate::tui::worker_bridge::IncomingTask;

pub use crate::session::SessionEvent;

const SLASH_COMMANDS: &[&str] = &[
    "/help",
    "/sessions",
    "/swarm",
    "/ralph",
    "/bus",
    "/protocol",
    "/model",
    "/settings",
    "/lsp",
    "/rlm",
    "/symbols",
    "/chat",
    "/new",
    "/keys",
];

#[derive(Default)]
pub struct App {
    pub state: AppState,
}

pub struct AppState {
    pub view_mode: ViewMode,
    pub input_mode: InputMode,
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub input_cursor: usize,
    pub input_scroll: usize,
    pub chat_scroll: usize,
    pub status: String,
    pub processing: bool,
    pub session_id: Option<String>,
    pub sessions: Vec<SessionSummary>,
    pub selected_session: usize,
    pub session_filter: String,
    pub cwd_display: String,
    pub bus_log: BusLogState,
    pub swarm: SwarmViewState,
    pub ralph: RalphViewState,
    pub symbol_search: SymbolSearchState,
    pub slash_suggestions: Vec<String>,
    pub selected_slash_suggestion: usize,
    pub command_history: Vec<String>,
    pub history_index: Option<usize>,
    pub worker_id: Option<String>,
    pub worker_name: Option<String>,
    pub a2a_connected: bool,
    pub recent_tasks: Vec<String>,
    pub worker_bridge_registered_agents: HashSet<String>,
    pub worker_bridge_processing_state: Option<bool>,
    pub worker_task_queue: VecDeque<IncomingTask>,
    pub help_scroll: HelpScrollState,
    pub show_help: bool,
    pub available_models: Vec<String>,
    pub selected_model_index: usize,
    pub model_picker_active: bool,
    pub model_filter: String,
    pub streaming_text: String,
    pub processing_started_at: Option<Instant>,
    pub worker_autorun_enabled: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            view_mode: ViewMode::Chat,
            input_mode: InputMode::Normal,
            messages: vec![],
            input: String::new(),
            input_cursor: 0,
            input_scroll: 0,
            chat_scroll: 0,
            status: "Ready — type a message and press Enter. Ctrl+C/Ctrl+Q quits.".to_string(),
            processing: false,
            session_id: None,
            sessions: vec![],
            selected_session: 0,
            session_filter: String::new(),
            cwd_display: String::new(),
            bus_log: BusLogState::new(),
            swarm: SwarmViewState::new(),
            ralph: RalphViewState::new(),
            symbol_search: SymbolSearchState::new(),
            slash_suggestions: vec![],
            selected_slash_suggestion: 0,
            command_history: Vec::new(),
            history_index: None,
            worker_id: None,
            worker_name: None,
            a2a_connected: false,
            recent_tasks: Vec::new(),
            worker_bridge_registered_agents: HashSet::new(),
            worker_bridge_processing_state: None,
            worker_task_queue: VecDeque::new(),
            help_scroll: HelpScrollState::default(),
            show_help: false,
            available_models: Vec::new(),
            selected_model_index: 0,
            model_picker_active: false,
            model_filter: String::new(),
            streaming_text: String::new(),
            processing_started_at: None,
            worker_autorun_enabled: true,
        }
    }
}

impl AppState {
    fn input_char_count(&self) -> usize {
        self.input.chars().count()
    }

    fn char_to_byte_index(&self, char_index: usize) -> usize {
        if char_index == 0 {
            return 0;
        }
        self.input
            .char_indices()
            .nth(char_index)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.input.len())
    }

    fn char_at(&self, char_index: usize) -> Option<char> {
        self.input.chars().nth(char_index)
    }

    pub fn clamp_input_cursor(&mut self) {
        self.input_cursor = self.input_cursor.min(self.input_char_count());
    }

    pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
        if visible_width == 0 {
            self.input_scroll = self.input_cursor;
            return;
        }
        if self.input_cursor < self.input_scroll {
            self.input_scroll = self.input_cursor;
        } else if self.input_cursor >= self.input_scroll.saturating_add(visible_width) {
            self.input_scroll = self
                .input_cursor
                .saturating_sub(visible_width.saturating_sub(1));
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor < self.input_char_count() {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_word_left(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor > 0
            && self
                .char_at(self.input_cursor - 1)
                .is_some_and(char::is_whitespace)
        {
            self.input_cursor -= 1;
        }
        while self.input_cursor > 0
            && self
                .char_at(self.input_cursor - 1)
                .is_some_and(|c| !c.is_whitespace())
        {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_word_right(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor < self.input_char_count()
            && self
                .char_at(self.input_cursor)
                .is_some_and(char::is_whitespace)
        {
            self.input_cursor += 1;
        }
        while self.input_cursor < self.input_char_count()
            && self
                .char_at(self.input_cursor)
                .is_some_and(|c| !c.is_whitespace())
        {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor = self.input_char_count();
    }

    pub fn insert_char(&mut self, c: char) {
        self.clamp_input_cursor();
        let byte_index = self.char_to_byte_index(self.input_cursor);
        self.input.insert(byte_index, c);
        self.input_cursor += 1;
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn delete_backspace(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor == 0 {
            return;
        }
        let start = self.char_to_byte_index(self.input_cursor - 1);
        let end = self.char_to_byte_index(self.input_cursor);
        self.input.replace_range(start..end, "");
        self.input_cursor -= 1;
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn delete_forward(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor >= self.input_char_count() {
            return;
        }
        let start = self.char_to_byte_index(self.input_cursor);
        let end = self.char_to_byte_index(self.input_cursor + 1);
        self.input.replace_range(start..end, "");
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.chat_scroll = self.chat_scroll.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.chat_scroll = self.chat_scroll.saturating_add(amount);
    }

    pub fn scroll_to_bottom(&mut self) {
        // Sentinel value — clamped to actual content height at render time.
        self.chat_scroll = 1_000_000;
    }

    pub fn sessions_select_prev(&mut self) {
        if self.selected_session > 0 {
            self.selected_session -= 1;
        }
    }

    pub fn sessions_select_next(&mut self) {
        if self.selected_session + 1 < self.filtered_sessions().len() {
            self.selected_session += 1;
        }
    }

    pub fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
        if self.session_filter.is_empty() {
            self.sessions.iter().enumerate().collect()
        } else {
            let filter = self.session_filter.to_lowercase();
            self.sessions
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    s.title
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&filter)
                        || s.id.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    pub fn clear_session_filter(&mut self) {
        self.session_filter.clear();
        self.selected_session = 0;
    }

    pub fn session_filter_backspace(&mut self) {
        self.session_filter.pop();
        self.selected_session = 0;
    }

    pub fn session_filter_push(&mut self, c: char) {
        self.session_filter.push(c);
        self.selected_session = 0;
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode;
    }

    pub fn set_worker_bridge(&mut self, worker_id: String, worker_name: String) {
        self.worker_id = Some(worker_id);
        self.worker_name = Some(worker_name);
        self.a2a_connected = true;
    }

    pub fn sync_worker_bridge_processing(&mut self, processing: bool) {
        self.worker_bridge_processing_state = Some(processing);
    }

    pub fn register_worker_agent(&mut self, name: String) {
        self.worker_bridge_registered_agents.insert(name);
    }

    pub fn push_recent_task(&mut self, task: String) {
        self.recent_tasks.push(task);
        if self.recent_tasks.len() > 20 {
            let drain_len = self.recent_tasks.len() - 20;
            self.recent_tasks.drain(0..drain_len);
        }
    }

    pub fn enqueue_worker_task(&mut self, task: IncomingTask) {
        self.worker_task_queue.push_back(task);
        while self.worker_task_queue.len() > 20 {
            self.worker_task_queue.pop_front();
        }
    }

    pub fn dequeue_worker_task(&mut self) -> Option<IncomingTask> {
        self.worker_task_queue.pop_front()
    }

    pub fn refresh_slash_suggestions(&mut self) {
        if !self.input.starts_with('/') {
            self.slash_suggestions.clear();
            self.selected_slash_suggestion = 0;
            return;
        }

        let query = self.input.trim().to_lowercase();
        self.slash_suggestions = SLASH_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&query))
            .map(|cmd| (*cmd).to_string())
            .collect();

        if self.slash_suggestions.is_empty() && query == "/" {
            self.slash_suggestions = SLASH_COMMANDS
                .iter()
                .map(|cmd| (*cmd).to_string())
                .collect();
        }

        if self.selected_slash_suggestion >= self.slash_suggestions.len() {
            self.selected_slash_suggestion = self.slash_suggestions.len().saturating_sub(1);
        }
    }

    pub fn slash_suggestions_visible(&self) -> bool {
        self.view_mode == ViewMode::Chat && !self.processing && self.input.starts_with('/')
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

    pub fn filtered_models(&self) -> Vec<&str> {
        if self.model_filter.is_empty() {
            self.available_models.iter().map(String::as_str).collect()
        } else {
            let filter = self.model_filter.to_lowercase();
            self.available_models
                .iter()
                .map(String::as_str)
                .filter(|model| model.to_lowercase().contains(&filter))
                .collect()
        }
    }

    pub fn set_available_models(&mut self, models: Vec<String>) {
        self.available_models = models;
        if self.selected_model_index >= self.filtered_models().len() {
            self.selected_model_index = self.filtered_models().len().saturating_sub(1);
        }
    }

    pub fn open_model_picker(&mut self) {
        self.model_picker_active = true;
        self.model_filter.clear();
        self.selected_model_index = 0;
        self.status = "Model picker".to_string();
    }

    pub fn close_model_picker(&mut self) {
        self.model_picker_active = false;
        self.model_filter.clear();
        self.selected_model_index = 0;
    }

    pub fn model_select_prev(&mut self) {
        self.selected_model_index = self.selected_model_index.saturating_sub(1);
    }

    pub fn model_select_next(&mut self) {
        if self.selected_model_index + 1 < self.filtered_models().len() {
            self.selected_model_index += 1;
        }
    }

    pub fn model_filter_push(&mut self, c: char) {
        self.model_filter.push(c);
        self.selected_model_index = 0;
    }

    pub fn model_filter_backspace(&mut self) {
        self.model_filter.pop();
        self.selected_model_index = 0;
    }

    pub fn selected_model(&self) -> Option<&str> {
        self.filtered_models()
            .get(self.selected_model_index)
            .copied()
    }

    pub async fn refresh_available_models(
        &mut self,
        registry: Option<&Arc<ProviderRegistry>>,
    ) -> anyhow::Result<()> {
        let mut models = Vec::new();
        let Some(registry) = registry else {
            self.available_models.clear();
            return Ok(());
        };

        for provider_name in registry.list() {
            if let Some(provider) = registry.get(provider_name)
                && let Ok(provider_models) = provider.list_models().await
            {
                models.extend(provider_models.into_iter().map(|model| {
                    if model.id.contains('/') {
                        model.id
                    } else {
                        format!("{provider_name}/{}", model.id)
                    }
                }));
            }
        }

        models.sort();
        models.dedup();
        self.set_available_models(models);
        Ok(())
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.input_cursor = 0;
        self.input_scroll = 0;
        self.input_mode = InputMode::Normal;
        self.slash_suggestions.clear();
        self.selected_slash_suggestion = 0;
        self.history_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;

    #[test]
    fn utf8_cursor_moves_by_character_not_byte() {
        let mut state = AppState {
            input: "aé🙂z".to_string(),
            ..Default::default()
        };

        state.move_cursor_end();
        assert_eq!(state.input_cursor, 4);

        state.move_cursor_left();
        assert_eq!(state.input_cursor, 3);
        state.move_cursor_left();
        assert_eq!(state.input_cursor, 2);
        state.move_cursor_right();
        assert_eq!(state.input_cursor, 3);
    }

    #[test]
    fn utf8_backspace_removes_full_character() {
        let mut state = AppState {
            input: "aé🙂z".to_string(),
            ..Default::default()
        };

        state.move_cursor_end();
        state.move_cursor_left();
        state.delete_backspace();

        assert_eq!(state.input, "aéz");
        assert_eq!(state.input_cursor, 2);
    }

    #[test]
    fn utf8_delete_forward_removes_full_character() {
        let mut state = AppState {
            input: "aé🙂z".to_string(),
            input_cursor: 1,
            ..Default::default()
        };

        state.delete_forward();

        assert_eq!(state.input, "a🙂z");
        assert_eq!(state.input_cursor, 1);
    }

    #[test]
    fn utf8_insert_respects_character_cursor() {
        let mut state = AppState {
            input: "a🙂z".to_string(),
            input_cursor: 2,
            ..Default::default()
        };

        state.insert_char('é');

        assert_eq!(state.input, "a🙂éz");
        assert_eq!(state.input_cursor, 3);
    }

    #[test]
    fn model_filter_limits_visible_models() {
        let mut state = AppState {
            available_models: vec![
                "zai/glm-5".to_string(),
                "openai/gpt-4o".to_string(),
                "openrouter/qwen/qwen3-coder".to_string(),
            ],
            ..Default::default()
        };

        state.model_filter_push('g');
        state.model_filter_push('p');
        state.model_filter_push('t');

        assert_eq!(state.filtered_models(), vec!["openai/gpt-4o"]);
        assert_eq!(state.selected_model(), Some("openai/gpt-4o"));
    }
}

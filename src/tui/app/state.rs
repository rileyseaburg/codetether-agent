use crate::session::SessionSummary;
use crate::tui::bus_log::BusLogState;
use crate::tui::chat::message::ChatMessage;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::RalphViewState;
use crate::tui::swarm_view::SwarmViewState;
use crate::tui::symbol_search::SymbolSearchState;

pub use crate::session::SessionEvent;

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
    pub cwd_display: String,
    pub bus_log: BusLogState,
    pub swarm: SwarmViewState,
    pub ralph: RalphViewState,
    pub symbol_search: SymbolSearchState,
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
            cwd_display: String::new(),
            bus_log: BusLogState::new(),
            swarm: SwarmViewState::new(),
            ralph: RalphViewState::new(),
            symbol_search: SymbolSearchState::new(),
        }
    }
}

impl AppState {
    pub fn clamp_input_cursor(&mut self) {
        self.input_cursor = self.input_cursor.min(self.input.len());
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
        if self.input_cursor < self.input.len() {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_word_left(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor > 0
            && self.input.as_bytes()[self.input_cursor - 1].is_ascii_whitespace()
        {
            self.input_cursor -= 1;
        }
        while self.input_cursor > 0
            && !self.input.as_bytes()[self.input_cursor - 1].is_ascii_whitespace()
        {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_word_right(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor < self.input.len()
            && self.input.as_bytes()[self.input_cursor].is_ascii_whitespace()
        {
            self.input_cursor += 1;
        }
        while self.input_cursor < self.input.len()
            && !self.input.as_bytes()[self.input_cursor].is_ascii_whitespace()
        {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor = self.input.len();
    }

    pub fn insert_char(&mut self, c: char) {
        self.clamp_input_cursor();
        self.input.insert(self.input_cursor, c);
        self.input_cursor += c.len_utf8();
    }

    pub fn delete_backspace(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor == 0 {
            return;
        }
        self.input_cursor -= 1;
        self.input.remove(self.input_cursor);
    }

    pub fn delete_forward(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor >= self.input.len() {
            return;
        }
        self.input.remove(self.input_cursor);
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.chat_scroll = self.chat_scroll.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.chat_scroll = self.chat_scroll.saturating_add(amount);
    }

    pub fn scroll_to_bottom(&mut self) {
        self.chat_scroll = usize::MAX / 2;
    }

    pub fn sessions_select_prev(&mut self) {
        if self.selected_session > 0 {
            self.selected_session -= 1;
        }
    }

    pub fn sessions_select_next(&mut self) {
        if self.selected_session + 1 < self.sessions.len() {
            self.selected_session += 1;
        }
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode;
    }
}

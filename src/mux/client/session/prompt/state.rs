//! Editable character buffer for the mux control prompt.

pub(super) struct State {
    chars: Vec<char>,
    cursor: usize,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            chars: Vec::new(),
            cursor: 0,
        }
    }
    pub(super) fn line(&self) -> String {
        self.chars.iter().collect()
    }
    pub(super) fn prefix(&self) -> String {
        self.chars[..self.cursor].iter().collect()
    }
    pub(super) fn insert(&mut self, value: char) {
        self.chars.insert(self.cursor, value);
        self.cursor += 1;
    }
    pub(super) fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.chars.remove(self.cursor);
        }
    }
    pub(super) fn delete(&mut self) {
        if self.cursor < self.chars.len() {
            self.chars.remove(self.cursor);
        }
    }
    pub(super) fn left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
    pub(super) fn right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.chars.len());
    }
    pub(super) fn home(&mut self) {
        self.cursor = 0;
    }
    pub(super) fn end(&mut self) {
        self.cursor = self.chars.len();
    }
    pub(super) fn replace(&mut self, line: String) {
        self.chars = line.chars().collect();
        self.cursor = self.chars.len();
    }
}

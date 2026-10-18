//! Character insertion, deletion and whole-line replacement.
use super::state::State;
impl State {
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

    pub(super) fn replace(&mut self, line: String) {
        self.chars = line.chars().collect();
        self.cursor = self.chars.len();
    }
}

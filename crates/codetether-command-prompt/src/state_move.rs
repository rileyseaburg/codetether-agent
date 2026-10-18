//! Cursor movement bounded by the editable character buffer.
use super::state::State;
impl State {
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
}

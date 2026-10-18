//! Character-buffer storage and prompt text access.
//! Editable character buffer for the mux control prompt.

pub(super) struct State {
    pub(super) chars: Vec<char>,
    pub(super) cursor: usize,
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
}

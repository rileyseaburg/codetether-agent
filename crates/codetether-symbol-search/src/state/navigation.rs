//! Symbol picker state navigation.
use super::SymbolSearchState;
use crate::SymbolEntry;

impl SymbolSearchState {
    /// Returns the selected symbol, if the index is valid.
    pub fn selected_symbol(&self) -> Option<&SymbolEntry> {
        self.results.get(self.selected)
    }
    /// Appends a query character and resets selection.
    pub fn handle_char(&mut self, character: char) {
        self.query.push(character);
        self.selected = 0;
    }
    /// Removes the final query character and resets selection.
    pub fn handle_backspace(&mut self) {
        self.query.pop();
        self.selected = 0;
    }
    /// Moves selection up, saturating at the first result.
    pub fn select_prev(&mut self) {
        self.select(self.selected.saturating_sub(1));
    }
    /// Moves selection down, saturating at the last result.
    pub fn select_next(&mut self) {
        let last = self.results.len().saturating_sub(1);
        self.select((self.selected + 1).min(last));
    }
    fn select(&mut self, index: usize) {
        if !self.results.is_empty() {
            self.selected = index;
            self.list_state.select(Some(index));
        }
    }
}

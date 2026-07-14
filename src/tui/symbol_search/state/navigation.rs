use super::SymbolSearchState;
use crate::tui::symbol_search::SymbolEntry;

impl SymbolSearchState {
    pub fn selected_symbol(&self) -> Option<&SymbolEntry> {
        self.results.get(self.selected)
    }
    pub fn handle_char(&mut self, character: char) {
        self.query.push(character);
        self.selected = 0;
    }
    pub fn handle_backspace(&mut self) {
        self.query.pop();
        self.selected = 0;
    }
    pub fn select_prev(&mut self) {
        self.select(self.selected.saturating_sub(1));
    }
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

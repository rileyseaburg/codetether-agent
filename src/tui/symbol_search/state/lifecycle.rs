use super::{SymbolSearchMode, SymbolSearchState};
use crate::tui::symbol_search::SymbolEntry;

impl SymbolSearchState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(&mut self) {
        self.activate(SymbolSearchMode::Navigate);
    }
    pub fn open_mention(&mut self, start: usize) {
        self.activate(SymbolSearchMode::Mention { start });
    }
    pub fn close(&mut self) {
        *self = Self::default();
    }
    pub fn set_results(&mut self, results: Vec<SymbolEntry>) {
        self.results = results;
        self.selected = 0;
        self.loading = false;
        self.error = None;
        self.list_state
            .select((!self.results.is_empty()).then_some(0));
    }
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.results.clear();
        self.loading = false;
    }
    fn activate(&mut self, mode: SymbolSearchMode) {
        *self = Self {
            active: true,
            mode,
            ..Self::default()
        };
    }
}

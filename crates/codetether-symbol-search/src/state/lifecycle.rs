//! Symbol picker state lifecycle.
use super::{SymbolSearchMode, SymbolSearchState};
use crate::SymbolEntry;

impl SymbolSearchState {
    /// Creates an inactive picker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Opens the picker for source navigation.
    pub fn open(&mut self) {
        self.activate(SymbolSearchMode::Navigate);
    }
    /// Opens the picker to replace the mention marker at `start`.
    pub fn open_mention(&mut self, start: usize) {
        self.activate(SymbolSearchMode::Mention { start });
    }
    /// Closes the picker and clears its state.
    pub fn close(&mut self) {
        *self = Self::default();
    }
    /// Accepts results, selects the first and clears loading and errors.
    pub fn set_results(&mut self, results: Vec<SymbolEntry>) {
        self.results = results;
        self.selected = 0;
        self.loading = false;
        self.error = None;
        self.list_state
            .select((!self.results.is_empty()).then_some(0));
    }
    /// Records a search error, clears results and ends loading.
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

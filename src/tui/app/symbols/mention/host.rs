//! Bind mention operations to the application's existing editor.
use crate::tui::app::state::App;
use crate::tui::symbol_search::SymbolSearchState;
use std::ops::Range;
impl codetether_symbol_mention::Composer for App {
    fn input(&self) -> &str {
        &self.state.input
    }
    fn cursor(&self) -> usize {
        self.state.input_cursor
    }
    fn picker(&self) -> &SymbolSearchState {
        &self.state.symbol_search
    }
    fn picker_mut(&mut self) -> &mut SymbolSearchState {
        &mut self.state.symbol_search
    }
    fn insert_char(&mut self, character: char) {
        self.state.insert_char(character);
    }
    fn insert_text(&mut self, text: &str) {
        self.state.insert_text(text);
    }
    fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        self.state.replace_input_chars(range, replacement);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
}

//! Bind mention operations to the application's existing editor.
use crate::tui::app::state::App;
use crate::tui::symbol_search::SymbolSearchState;
use std::ops::Range;
impl codetether_symbol_mention::Composer for App {
    fn input(&self) -> &str {
        &self.state.composer.input
    }
    fn cursor(&self) -> usize {
        self.state.composer.cursor
    }
    fn picker(&self) -> &SymbolSearchState {
        &self.state.symbol_search
    }
    fn picker_mut(&mut self) -> &mut SymbolSearchState {
        &mut self.state.symbol_search
    }
    fn insert_char(&mut self, character: char) {
        self.state.composer.insert_char(character);
    }
    fn insert_text(&mut self, text: &str) {
        self.state.composer.insert_text(text);
    }
    fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        self.state.composer.replace_input_chars(range, replacement);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
}

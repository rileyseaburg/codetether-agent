//! Native symbol search and view-filter bindings for shared paste routing.
use crate::tui::{
    app::{state::App, symbols},
    models::ViewMode,
};
use codetether_paste_buffer::routing::{Host, View};
impl Host for App {
    fn symbol_search_active(&self) -> bool {
        symbols::symbol_search_active(self)
    }
    fn symbol_char(&mut self, ch: char) {
        self.state.symbol_search.handle_char(ch);
    }
    fn refresh_symbols(&mut self) {
        symbols::schedule_refresh(self);
    }
    fn view(&self) -> View {
        match &self.state.view_mode {
            ViewMode::Bus => View::Bus,
            ViewMode::Model => View::Model,
            ViewMode::Sessions => View::Sessions,
            ViewMode::Chat => View::Chat,
            _ => View::Other,
        }
    }
    fn bus_filter_active(&self) -> bool {
        self.state.bus_log.filter_input_mode
    }
    fn bus_char(&mut self, ch: char) {
        self.state.bus_log.push_filter_char(ch);
    }
    fn bus_filter(&self) -> &str {
        &self.state.bus_log.filter
    }
    fn model_char(&mut self, ch: char) {
        self.state.model_filter_push(ch);
    }
    fn session_char(&mut self, ch: char) {
        self.state.session_filter_push(ch);
    }
}

//! Native effects shared by character and backspace routing.
mod buffer;
mod filters;
mod symbols;
use crate::tui::{
    app::state::App,
    models::{InputMode, ViewMode},
};
use codetether_tui_navigation::text::{Action, Host};
pub(super) struct Native<'a>(pub &'a mut App);
impl Host for Native<'_> {
    fn symbols_active(&self) -> bool {
        crate::tui::app::symbols::symbol_search_active(self.0)
    }
    fn in_view(&self, view: ViewMode) -> bool {
        self.0.state.view_mode == view
    }
    fn bus_filter_active(&self) -> bool {
        self.0.state.bus_log.filter_input_mode
    }
    fn bus_filter(&self) -> &str {
        &self.0.state.bus_log.filter
    }
    fn input(&self) -> &str {
        &self.0.state.input
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn set_mode(&mut self, mode: InputMode) {
        self.0.state.input_mode = mode;
    }
    fn effect(&mut self, action: Action) {
        if buffer::apply(self.0, &action)
            || filters::apply(self.0, &action)
            || symbols::apply(self.0, &action)
        {
            return;
        }
        unreachable!("unmapped text input effect");
    }
}

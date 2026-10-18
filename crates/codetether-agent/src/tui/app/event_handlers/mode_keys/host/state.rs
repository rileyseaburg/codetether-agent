use super::Native;
use crate::tui::{app::input, models::ViewMode};
use codetether_tui_navigation::modes::{Action, State};
impl State for Native<'_> {
    fn in_view(&self, view: ViewMode) -> bool {
        self.app.state.view.mode == view
    }
    fn effect(&mut self, action: Action) {
        match action {
            Action::Attach => {
                crate::tui::app::file_picker::file_picker_attach(self.app);
            }
            Action::FilterBackspace => {
                self.app.state.session_picker.session_filter_backspace();
            }
            Action::BusG => {
                input::handle_bus_g(self.app);
            }
            Action::BusC => {
                input::handle_bus_c(self.app);
            }
            Action::BusFilter => {
                input::handle_bus_slash(self.app);
            }
            Action::AuditFilter => {
                self.app.state.audit.toggle_filter();
            }
        }
    }
}

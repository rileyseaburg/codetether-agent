//! Native picker, chat return and session model bindings.
use crate::tui::models::ViewMode;
use crate::{session::Session, tui::app::state::App};
use codetether_model_picker::ModelPickerState;
pub(super) struct Picker<'a>(pub(super) &'a mut App);
impl codetether_model_picker::interactive::View for Picker<'_> {
    fn picker(&mut self) -> &mut ModelPickerState {
        &mut self.0.state.model_picker
    }
    fn show_picker(&mut self) {
        self.0.state.set_view_mode(ViewMode::Model);
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn return_to_chat(&mut self) {
        crate::tui::app::session_sync::return_to_chat(self.0);
    }
}
impl codetether_model_picker::interactive::Session for Session {
    fn set_model(&mut self, model: String) {
        self.metadata.model = Some(model);
    }
}

impl codetether_model_picker::interactive::verifier_command::CommandView for Picker<'_> {
    fn set_verifier_model(&mut self, model: Option<&str>) {
        crate::tool::goal::verify::set_verifier_model(model);
    }
    fn selected_verifier_model(&self) -> Option<String> {
        crate::tool::goal::verify::selected_verifier_model()
    }
    fn clear_input(&mut self) {
        self.0.state.composer.clear_input();
    }
}

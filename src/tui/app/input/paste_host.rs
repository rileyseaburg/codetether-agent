//! Native composer operations used by shared chat paste handling.
use crate::tui::{app::state::App, models::InputMode};
use codetether_paste_buffer::PendingTextPaste;
impl codetether_paste_buffer::input::Host for App {
    fn try_attach_image(&mut self, text: &str) -> bool {
        super::super::image_data_paste::try_attach_data_url(self, text)
    }
    fn input(&self) -> &str {
        &self.state.input
    }
    fn set_command_mode(&mut self, command: bool) {
        self.state.input_mode = if command {
            InputMode::Command
        } else {
            InputMode::Editing
        };
    }
    fn insert_text(&mut self, text: &str) {
        self.state.insert_text(text);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn pending_pastes(&mut self) -> &mut Vec<PendingTextPaste> {
        &mut self.state.pending_text_pastes
    }
}

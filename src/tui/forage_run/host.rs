//! Interactive forage presentation and application backend selection.
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use codetether_forage_interactive::{ForageState, View};
pub(super) type Runtime =
    codetether_native_tui::runs::Forage<crate::forage::execution_host::Native>;
impl View for App {
    fn state(&mut self) -> &mut ForageState {
        &mut self.state.forage
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn push_system(&mut self, text: String) {
        self.state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
}

//! Interactive swarm presentation and application backend selection.
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
    models::ViewMode,
};
use codetether_swarm_view::SwarmViewState;
use codetether_swarm_view::interactive::View;
pub(super) type Runtime = codetether_native_tui::runs::Swarm<
    codetether_native_swarm::ExecutorHost<
        crate::tool::registry_lifecycle::Native,
        crate::agent::Agent,
    >,
>;
impl View for App {
    fn swarm(&mut self) -> &mut SwarmViewState {
        &mut self.state.swarm
    }
    fn show_monitor(&mut self) {
        crate::tui::app::state::view::transition(&mut self.state).set_view_mode(ViewMode::Swarm);
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
        self.state.chat_viewport.scroll_to_bottom();
    }
}

//! Native state and worker bindings for shared objective approval.
use crate::tui::{
    app::{
        autochat::{events::AutochatUiEvent, worker::start_autochat_relay},
        state::App,
    },
    chat::message::{ChatMessage, MessageType},
};
use codetether_okr::OkrRepository;
use codetether_relay_okr::PendingOkrApproval;
use std::sync::Arc;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_relay_okr::interactive::approval::Host for Native<'_> {
    type Receiver = tokio::sync::mpsc::UnboundedReceiver<AutochatUiEvent>;
    fn pending(&mut self) -> &mut Option<PendingOkrApproval> {
        &mut self.0.state.pending_okr_approval
    }
    fn repository(&self) -> Option<&Arc<OkrRepository>> {
        self.0.state.okr_repository.as_ref()
    }
    fn start_relay(task: String, model: String) -> Self::Receiver {
        start_autochat_relay(task, model)
    }
    fn set_relay_running(&mut self, running: bool) {
        self.0.state.autochat.running = running;
    }
    fn set_relay_receiver(&mut self, receiver: Self::Receiver) {
        self.0.state.autochat.rx = Some(receiver);
    }
    fn push_system(&mut self, text: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.scroll_to_bottom();
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}

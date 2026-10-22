//! Native bus observers, ownership checks and chat presentation.
use super::Native;
use crate::tui::chat::message::{ChatMessage, MessageType};
use codetether_bus_core::{BusEnvelope, BusMessage};
use codetether_worker_events::IncomingTask;
impl codetether_bus_view::ingress::Effects for Native<'_> {
    fn observe_tasks(&mut self, message: &BusMessage) {
        self.0.state.active_tasks.observe(message);
    }
    fn observe_tools(&mut self, message: &BusMessage) {
        self.0.state.tool_calls.observe(message);
    }
    fn ingest_log(&mut self, envelope: &BusEnvelope) {
        self.0.state.bus_log.ingest(envelope);
    }
    fn enqueue_task(&mut self, task: IncomingTask) {
        self.0.state.worker.enqueue_worker_task(task);
    }
    fn owns_agent(&self, parent: &str, agent: &str) -> bool {
        crate::tui::app::managed_agent::owned(parent, agent)
    }
    fn knows_peer(&self, parent: &str, peer: &str) -> bool {
        codetether_peer_agent::observation::transcript(peer, parent).is_some()
    }
    fn push_assistant(&mut self, text: String) {
        self.0
            .state
            .messages
            .push(ChatMessage::new(MessageType::Assistant, text));
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.chat_viewport.scroll_to_bottom();
    }
}

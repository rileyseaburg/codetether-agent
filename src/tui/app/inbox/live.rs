//! Native parked-turn admission and reply settlement bindings.
use crate::tui::app::{session_runtime::SessionSlot, state::App};
use codetether_a2a_inbox::Inbound;
use codetether_worker_events::IncomingTask;
impl codetether_a2a_interactive::Inbox for App {
    fn processing(&self) -> bool {
        self.state.processing
    }
    fn inbound(&mut self) -> &mut Option<Inbound> {
        &mut self.state.live_inbound
    }
    fn enqueue_task(&mut self, task: IncomingTask) {
        self.state.enqueue_worker_task(task);
    }
}
pub(crate) fn admit(app: &mut App) {
    codetether_a2a_interactive::live::admit(app);
}
pub(crate) fn settle(app: &mut App, outcome: Result<String, String>) {
    codetether_a2a_interactive::live::settle(&mut app.state.live_inbound, outcome);
}
pub(crate) fn settle_from_session(app: &mut App, slot: &SessionSlot) {
    codetether_a2a_interactive::live::settle_from_session(
        &mut app.state.live_inbound,
        slot.borrow().map(|session| session.messages.as_slice()),
    );
}

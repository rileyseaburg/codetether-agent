//! Bind shared swarm commands to the native executor and TUI.
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmControl, SwarmExecutor};
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
    models::ViewMode,
};
use codetether_swarm_view::interactive::{Host, View};
use codetether_swarm_view::{SwarmEvent, SwarmViewState};
pub(super) struct Runtime;
impl Host for Runtime {
    async fn execute(
        config: SwarmConfig,
        control: SwarmControl,
        events: tokio::sync::mpsc::Sender<SwarmEvent>,
        task: String,
        strategy: DecompositionStrategy,
    ) -> Result<(), String> {
        let mut executor = SwarmExecutor::new(config)
            .with_event_tx(events)
            .with_control(control);
        if let Some(bus) = crate::bus::global::global() {
            executor = executor.with_bus(bus);
        }
        executor
            .execute(&task, strategy)
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}
impl View for App {
    fn swarm(&mut self) -> &mut SwarmViewState {
        &mut self.state.swarm
    }
    fn show_monitor(&mut self) {
        self.state.set_view_mode(ViewMode::Swarm);
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

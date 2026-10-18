//! Native registry and UI operations for shared detach commands.
use crate::{
    session::Session,
    tui::{
        app::state::{App, SpawnedAgent},
        models::ViewMode,
    },
};
impl codetether_agent_command::detach::Host for App {
    type Session = Session;
    fn is_processing(&self) -> bool {
        self.state.processing
    }
    fn contains_name(&self, name: &str) -> bool {
        self.state.agent_tree.spawned.contains_key(name)
    }
    fn current_model_id(&self) -> Option<String> {
        super::super::spawn_agent::model::current_model_id(self)
    }
    fn register(&mut self, name: String, agent: SpawnedAgent) {
        self.state.agent_tree.spawned.insert(name, agent);
    }
    fn resume_window(&self) -> usize {
        super::super::resume_window::session_resume_window()
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn show_subagents(&mut self) {
        crate::tui::app::state::view::transition(&mut self.state)
            .set_view_mode(ViewMode::Subagents);
    }
    fn push_system(&mut self, content: String) {
        self.state
            .messages
            .push(codetether_chat_model::ChatMessage::new(
                codetether_chat_model::MessageType::System,
                content,
            ));
    }
}

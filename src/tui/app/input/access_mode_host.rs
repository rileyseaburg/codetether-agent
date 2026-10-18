//! Native session and presentation effects for access-mode commands.
use super::{App, ProviderRegistry, Session};
use crate::{
    config::Config,
    tui::chat::message::{ChatMessage, MessageType},
};
use std::sync::Arc;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub session: &'a mut Session,
    pub registry: Option<&'a Arc<ProviderRegistry>>,
}
impl codetether_settings_policy::access_command::Host for Native<'_> {
    fn verifier_command(&mut self, prompt: &str) -> bool {
        super::verifier_model::run(self.app, self.registry, prompt)
    }
    fn apply_session_config(&mut self, config: &Config) {
        self.session
            .apply_config(config, self.registry.map(Arc::as_ref));
    }
    fn release_all(&mut self) -> Result<usize, String> {
        crate::tui::app::input::approval_command::release_all(self.app)
    }
    fn push_system(&mut self, text: String) {
        self.app
            .state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
    fn scroll_to_bottom(&mut self) {
        self.app.state.scroll_to_bottom();
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
}

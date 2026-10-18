use super::{App, Arc, Path, ProviderRegistry};
use crate::session::Session;
use crate::tui::app::{commands::handle_slash_command, input::codex_parity_command};
pub(super) struct Live<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub session: &'a mut Session,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
}
impl codetether_slash_command::intake::SessionHost for Live<'_> {
    async fn parity(&mut self, prompt: &str) -> bool {
        codex_parity_command::run(
            self.app,
            self.cwd,
            self.session,
            self.registry.as_ref(),
            prompt,
        )
        .await
    }
    async fn command(&mut self, prompt: &str) {
        handle_slash_command(
            self.app,
            self.cwd,
            self.session,
            self.registry.as_ref(),
            prompt,
        )
        .await;
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
}

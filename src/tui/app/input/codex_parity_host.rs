//! Bind utility routing to native session/workspace actions.
use super::*;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub session: &'a mut Session,
    pub registry: Option<&'a Arc<ProviderRegistry>>,
}
impl codetether_slash_command::utility::Host for Native<'_> {
    async fn access_mode(&mut self, prompt: &str) -> bool {
        access_mode_command::run(self.app, self.cwd, self.session, self.registry, prompt).await
    }
    async fn status(&mut self, label: &str) {
        status::show(self.app, self.cwd, self.session, label).await;
    }
    fn diff(&mut self) {
        diff::show(self.app, self.cwd);
    }
    fn copy(&mut self, prompt: &str) {
        copy::run(self.app, prompt);
    }
    fn review(&mut self) {
        codetether_slash_command::utility::review::prepare(self.app);
    }
    async fn dispatch(&mut self, command: &str) {
        crate::tui::app::commands::handle_slash_command(
            self.app,
            self.cwd,
            self.session,
            self.registry,
            command,
        )
        .await;
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
}

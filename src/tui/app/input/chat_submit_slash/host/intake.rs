use super::{Native, session::Live};
use crate::tui::app::input::approval_command;
impl codetether_slash_command::intake::Host for Native<'_> {
    async fn shell(&mut self, prompt: &str) -> bool {
        super::super::shell::run(self.app, self.cwd, prompt).await
    }
    fn approval(&mut self, prompt: &str) -> bool {
        approval_command::run(self.app, prompt)
    }
    async fn mux(&mut self, prompt: &str) -> bool {
        super::super::mux_command::run(self.app, self.cwd, prompt).await
    }
    async fn session(&mut self, prompt: &str) -> bool {
        let Some(session) = self.slot.borrow_mut() else {
            return false;
        };
        let mut live = Live {
            app: self.app,
            cwd: self.cwd,
            session,
            registry: self.registry,
        };
        codetether_slash_command::intake::with_session(&mut live, prompt).await;
        true
    }
    fn session_free(&mut self, prompt: &str) -> bool {
        super::super::slash_no_session::run(self.app, self.cwd, self.registry, prompt)
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
    fn status(&mut self, message: String) {
        self.app.state.status = message;
    }
}

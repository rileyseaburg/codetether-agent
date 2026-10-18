use super::Native;
use crate::tui::app::input;
impl codetether_chat_submit::Routing for Native<'_> {
    fn capture(&mut self) -> String {
        input::chat_submit_history::capture(self.app)
    }
    async fn managed(&mut self, prompt: &str) -> bool {
        crate::tui::app::managed_agent::chat::route(self.app, prompt).await
    }
    fn mentions(&self, prompt: &str) -> String {
        input::mention_route::route_prompt(prompt, self.app.state.active_spawned_agent.as_deref())
    }
    async fn continuation(&mut self, prompt: &str) -> bool {
        input::continue_command::run(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
            prompt,
        )
        .await
    }
    async fn slash(&mut self, prompt: &str) -> bool {
        input::chat_submit_slash::run(self.app, self.cwd, self.slot, self.registry, prompt).await
    }
}

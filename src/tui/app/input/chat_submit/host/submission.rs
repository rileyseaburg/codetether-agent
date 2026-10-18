use super::Native;
use crate::tui::app::input;
impl codetether_chat_submit::Submission for Native<'_> {
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    fn steer(&mut self, prompt: &str) {
        input::chat_steer::submit(self.app, self.runtime, prompt);
    }
    async fn finish(&mut self, prompt: &str) {
        input::chat_submit_finish::finish_submit(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
            prompt,
        )
        .await;
    }
}

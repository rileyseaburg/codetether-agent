use super::Native;
use crate::tui::app::input::chat_submit_dispatch::dispatch_prompt;
impl codetether_chat_submit::resume::Runtime for Native<'_> {
    async fn dispatch(&mut self, prompt: &str) {
        dispatch_prompt(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            prompt,
            Vec::new(),
            self.runtime,
        )
        .await;
    }
    async fn cancel(&self) {
        self.runtime.cancel_current().await;
    }
}

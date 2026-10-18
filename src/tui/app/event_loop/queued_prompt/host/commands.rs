use super::Native;
use crate::tui::app::input::goal_answer;
impl codetether_chat_submit::queued::Commands for Native<'_> {
    async fn commands(&mut self) {
        goal_answer::commands::drain(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
    async fn answers(&mut self) {
        goal_answer::deliver(self.app, self.slot, self.registry).await;
    }
}

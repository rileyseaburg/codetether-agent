use super::Native;
use codetether_tui_navigation::events::Review;
use crossterm::event::KeyEvent;
impl Review for Native<'_> {
    type Error = anyhow::Error;
    async fn review(&mut self, key: KeyEvent) -> anyhow::Result<Option<bool>> {
        super::super::super::answer_review_key::handle(self.app, self.slot, key).await
    }
    async fn resume(&mut self) {
        crate::tui::app::input::sessions::goal_autostart::resume(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
}

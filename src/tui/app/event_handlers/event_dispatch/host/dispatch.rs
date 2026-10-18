use super::Native;
use codetether_tui_navigation::events::Dispatch;
use crossterm::event::KeyEvent;
use std::time::Instant;
impl Dispatch for Native<'_> {
    async fn priority(&mut self, key: KeyEvent) -> Option<bool> {
        super::super::super::event_priority::handle(
            self.app,
            self.cwd,
            self.slot,
            self.runtime,
            key,
        )
        .await
    }
    fn modified(&mut self, key: KeyEvent) -> Option<anyhow::Result<bool>> {
        super::super::super::keyboard::handle_ctrl_key(self.app, self.cwd, self.runtime, key)
    }
    async fn unmodified(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
        super::super::super::keybinds::handle_unmodified_key(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
            key,
        )
        .await
    }
    fn last_key(&mut self) -> &mut Option<Instant> {
        &mut self.app.state.last_key_at
    }
}

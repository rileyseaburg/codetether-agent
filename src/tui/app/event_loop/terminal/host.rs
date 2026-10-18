use super::{App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge};
use crate::tui::app::event_handlers::{handle_event, handle_mouse_event, handle_paste_event};
use codetether_tui_navigation::terminal::Host;
use crossterm::event::{KeyEvent, MouseEvent};

pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
    pub worker_bridge: &'a Option<TuiWorkerBridge>,
    pub runtime: &'a TuiSessionHandle,
}
impl Host for Native<'_> {
    type Error = anyhow::Error;
    async fn key(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
        handle_event(
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
    fn editor_paste(&mut self, text: &str) -> bool {
        crate::tui::app::input::goal_answer::editor::paste(self.app, text)
    }
    async fn paste(&mut self, text: &str) {
        handle_paste_event(self.app, text).await;
    }
    async fn mouse(&mut self, mouse: MouseEvent) {
        handle_mouse_event(self.app, self.cwd, mouse).await;
    }
}

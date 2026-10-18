//! Bind existing events operations to shared dispatch.
use super::Native;
impl codetether_tui_reactor::dispatch::Events for Native<'_, '_> {
    type Terminal = std::io::Result<crossterm::event::Event>;
    type Session = crate::session::SessionEvent;
    async fn terminal(&mut self, event: Option<Self::Terminal>) -> anyhow::Result<bool> {
        let a = &mut *self.0;
        crate::tui::app::event_loop::terminal::handle_terminal_event(
            a.app,
            a.cwd,
            a.slot,
            a.registry,
            a.worker_bridge,
            a.runtime,
            event,
        )
        .await
    }
    async fn session(&mut self, event: Self::Session) {
        let a = &mut *self.0;
        crate::tui::app::session_events::handle_session_event(a.app, a.slot, a.worker_bridge, event)
            .await
    }
    async fn coalesce(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::coalesce::drain(a.app, a.slot, a.worker_bridge, a.io.event_rx)
            .await
    }
}

//! Bind existing after operations to shared dispatch.
use super::Native;
impl codetether_tui_reactor::dispatch::After for Native<'_, '_> {
    async fn drain(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::background::drain_background_updates(
            a.app,
            a.cwd,
            a.slot,
            a.bus_recorder,
            a.worker_bridge,
            a.io.event_rx,
            a.io.notice_rx,
        )
        .await
    }
    async fn queued(&mut self) {
        let a = &mut *self.0;
        super::super::queued_prompt::drain(
            a.app,
            a.cwd,
            a.slot,
            a.registry,
            a.worker_bridge,
            a.runtime,
        )
        .await
    }
    async fn inbox(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::bus_inbox::trigger_next(
            a.app,
            a.cwd,
            a.slot,
            a.registry,
            a.worker_bridge,
            a.runtime,
        )
        .await
    }
}

//! Bind existing notices operations to shared dispatch.
use super::Native;
impl codetether_tui_reactor::dispatch::Notices for Native<'_, '_> {
    type Notice = crate::tui::app::session_runtime::SessionNotice;
    async fn notice(&mut self, notice: Self::Notice) {
        let a = &mut *self.0;
        crate::tui::app::background::apply_single_notice(
            a.app,
            a.cwd,
            a.slot,
            a.worker_bridge,
            a.io.event_rx,
            notice,
        )
        .await
    }
    async fn reconnect(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::background::stream_reconnect_execute::execute(
            a.app, a.slot, a.registry, a.runtime,
        )
        .await
    }
    async fn smart_retry(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::smart_retry::execute_smart_switch_retry(
            a.app, a.slot, a.registry, a.runtime,
        )
        .await
    }
    async fn watchdog_retry(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::watchdog_retry::execute(a.app, a.slot, a.registry, a.runtime)
            .await
    }
}

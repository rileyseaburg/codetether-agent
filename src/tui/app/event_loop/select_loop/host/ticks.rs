//! Bind existing ticks operations to shared dispatch.
use super::Native;
impl codetether_tui_reactor::dispatch::Ticks for Native<'_, '_> {
    async fn watchdog(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::watchdog::maybe_watchdog_restart(
            a.app,
            a.runtime,
            a.timers.watchdog_interval,
        )
        .await
    }
    async fn audit(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::tick::refresh_audit(a.app).await
    }
    async fn tick(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::tick::run(a.app).await
    }
    async fn tick_retry(&mut self) {
        let a = &mut *self.0;
        crate::tui::app::event_loop::tick::check_and_retry(
            a.app,
            a.slot,
            a.registry,
            a.runtime,
            a.timers.watchdog_interval,
        )
        .await
    }
}

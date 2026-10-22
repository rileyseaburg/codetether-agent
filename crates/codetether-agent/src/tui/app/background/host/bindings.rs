use super::Native;
use codetether_tui_tick::background::Host;
impl Host for Native<'_> {
    type Notice = super::super::SessionNotice;
    fn models(&mut self) {
        self.app.state.model_picker.apply_refresh_status(
            self.app.state.view.mode == crate::tui::models::ViewMode::Model,
            &mut self.app.state.status,
        );
    }
    fn bus(&mut self) {
        if let Some(recorder) = self.bus_recorder {
            crate::tui::app::bus::ingest::drain(self.app, recorder);
        }
    }
    fn queue_workers(&mut self) {
        super::super::worker_tasks::queue_worker_tasks(self.app, self.worker_bridge);
    }
    async fn events(&mut self) {
        crate::tui::app::session_event_drain::drain_batch(
            self.app,
            self.slot,
            self.worker_bridge,
            self.event_rx,
        )
        .await;
    }
    fn next_notice(&mut self) -> Option<Self::Notice> {
        self.notice_rx.as_mut()?.try_recv().ok()
    }
    async fn before_notice(&mut self) {
        crate::tui::app::session_event_drain::drain_before_notice(
            self.app,
            self.slot,
            self.worker_bridge,
            self.event_rx,
        )
        .await;
    }
    async fn notice(&mut self, notice: Self::Notice) {
        super::super::notice::apply(self.app, self.cwd, self.slot, self.worker_bridge, notice)
            .await;
    }
}

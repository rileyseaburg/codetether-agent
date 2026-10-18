//! Bind existing pre-draw refresh, mux update and terminal rendering.
use super::Native;
impl codetether_tui_reactor::driver::Prepare for Native<'_> {
    fn before_draw(&mut self) {
        super::super::tick::before_draw(
            self.app,
            &self.worker_bridge,
            &mut self.setup.worker_sync_cursor,
        );
    }
    async fn update_mux(&mut self) {
        self.setup
            .mux_status
            .update(self.app, self.slot.view())
            .await;
    }
    fn draw(&mut self) -> anyhow::Result<()> {
        super::super::redraw::draw_if_ready(
            self.terminal,
            self.app,
            self.slot.view(),
            &mut self.last_draw,
        )
    }
}

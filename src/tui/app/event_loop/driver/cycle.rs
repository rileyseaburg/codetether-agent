//! Borrow native channels and session handles for one shared event cycle.
use super::super::{LoopIo, SelectArgs};
use super::Native;
impl codetether_tui_reactor::driver::Cycle for Native<'_> {
    async fn select(&mut self) -> anyhow::Result<bool> {
        let mut io = LoopIo::new(
            &mut self.event_rx,
            &mut self.notice_rx,
            &mut self.setup.shutdown_rx,
        );
        let mut args = SelectArgs {
            reader: &mut self.setup.reader,
            app: self.app,
            cwd: self.cwd,
            slot: self.slot,
            registry: &self.registry,
            worker_bridge: &mut self.worker_bridge,
            runtime: &self.runtime,
            io: &mut io,
            timers: &mut self.setup.timers,
            bus_recorder: &self.bus_recorder,
        };
        super::super::select_loop::select_once(&mut args).await
    }
    fn mark_redraw(&mut self) {
        self.app.state.needs_redraw = true;
    }
}

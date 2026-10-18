//! Per-iteration channel borrows and frontend event selection.
/// Implementation detail of [crate::declare_frontend_driver].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_driver_cycle {
    ($io:path, $args:path, $select:path) => {
        mod cycle {
            use super::Native;
            use $args as SelectArgs;
            use $io as LoopIo;
            impl $crate::driver::Cycle for Native<'_> {
                async fn select(&mut self) -> ::anyhow::Result<bool> {
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
                    $select(&mut args).await
                }
                fn mark_redraw(&mut self) {
                    self.app.state.needs_redraw = true;
                }
            }
        }
    };
}

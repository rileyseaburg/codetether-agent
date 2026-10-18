//! Frontend after operations for reactor dispatch.
/// Implementation detail of [crate::declare_frontend_selection].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_selection_after {
    ($drain:path, $queued:path, $inbox:path) => {
        mod after {
            use super::Native;
            impl $crate::dispatch::After for Native<'_, '_> {
                async fn drain(&mut self) {
                    let a = &mut *self.0;
                    $drain(
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
                    $queued(a.app, a.cwd, a.slot, a.registry, a.worker_bridge, a.runtime).await
                }
                async fn inbox(&mut self) {
                    let a = &mut *self.0;
                    $inbox(a.app, a.cwd, a.slot, a.registry, a.worker_bridge, a.runtime).await
                }
            }
        }
    };
}

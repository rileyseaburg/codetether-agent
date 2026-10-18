//! Frontend notices operations for reactor dispatch.
/// Implementation detail of [crate::declare_frontend_selection].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_selection_notices {
    ($notice_type:ty; $notice:path, $reconnect:path, $smart_retry:path, $watchdog_retry:path) => {
        mod notices {
            use super::Native;
            impl $crate::dispatch::Notices for Native<'_, '_> {
                type Notice = $notice_type;
                async fn notice(&mut self, notice: Self::Notice) {
                    let a = &mut *self.0;
                    $notice(a.app, a.cwd, a.slot, a.worker_bridge, a.io.event_rx, notice).await
                }
                async fn reconnect(&mut self) {
                    let a = &mut *self.0;
                    $reconnect(a.app, a.slot, a.registry, a.runtime).await
                }
                async fn smart_retry(&mut self) {
                    let a = &mut *self.0;
                    $smart_retry(a.app, a.slot, a.registry, a.runtime).await
                }
                async fn watchdog_retry(&mut self) {
                    let a = &mut *self.0;
                    $watchdog_retry(a.app, a.slot, a.registry, a.runtime).await
                }
            }
        }
    };
}

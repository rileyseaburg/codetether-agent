//! Frontend ticks operations for reactor dispatch.
/// Implementation detail of [crate::declare_frontend_selection].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_selection_ticks {
    ($watchdog:path, $audit:path, $tick:path, $retry:path) => {
        mod ticks {
            use super::Native;
            impl $crate::dispatch::Ticks for Native<'_, '_> {
                async fn watchdog(&mut self) {
                    let a = &mut *self.0;
                    $watchdog(a.app, a.runtime, a.timers.watchdog_interval).await
                }
                async fn audit(&mut self) {
                    let a = &mut *self.0;
                    $audit(a.app).await
                }
                async fn tick(&mut self) {
                    let a = &mut *self.0;
                    $tick(a.app).await
                }
                async fn tick_retry(&mut self) {
                    let a = &mut *self.0;
                    $retry(
                        a.app,
                        a.slot,
                        a.registry,
                        a.runtime,
                        a.timers.watchdog_interval,
                    )
                    .await
                }
            }
        }
    };
}

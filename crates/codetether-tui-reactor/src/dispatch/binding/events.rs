//! Frontend events operations for reactor dispatch.
/// Implementation detail of [crate::declare_frontend_selection].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_selection_events {
    ($terminal_type:ty, $event:ty; $terminal:path, $session:path, $coalesce:path) => {
        mod events {
            use super::Native;
            impl $crate::dispatch::Events for Native<'_, '_> {
                type Terminal = $terminal_type;
                type Session = $event;
                async fn terminal(
                    &mut self,
                    event: Option<Self::Terminal>,
                ) -> anyhow::Result<bool> {
                    let a = &mut *self.0;
                    $terminal(
                        a.app,
                        a.cwd,
                        a.slot,
                        a.registry,
                        a.worker_bridge,
                        a.runtime,
                        event,
                    )
                    .await
                }
                async fn session(&mut self, event: Self::Session) {
                    let a = &mut *self.0;
                    $session(a.app, a.slot, a.worker_bridge, event).await
                }
                async fn coalesce(&mut self) {
                    let a = &mut *self.0;
                    $coalesce(a.app, a.slot, a.worker_bridge, a.io.event_rx).await
                }
            }
        }
    };
}

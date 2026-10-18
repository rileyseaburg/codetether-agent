//! Frontend session selection and setting-toggle operations.
/// Implementation detail of [crate::declare_frontend_enter].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_enter_session {
    ($sessions:path, $settings:path) => {
        mod mode {
            use super::{
                App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge,
            };
            pub(super) async fn sessions(
                app: &mut App,
                cwd: &Path,
                slot: &mut SessionSlot,
                registry: &Option<Arc<ProviderRegistry>>,
                worker_bridge: &Option<TuiWorkerBridge>,
                runtime: &TuiSessionHandle,
            ) {
                $sessions(app, cwd, slot, registry, worker_bridge, runtime).await;
            }

            pub(super) async fn settings(app: &mut App, slot: &mut SessionSlot) {
                if let Some(session) = slot.borrow_mut() {
                    $settings(app, session).await;
                }
            }
        }
    };
}

//! Frontend swarm detail versus chat-submission behavior.
/// Implementation detail of [crate::declare_frontend_enter].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_enter_swarm {
    ($chat:path) => {
        mod enter_swarm {
            use super::{
                App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge,
            };
            /// Route Enter while in the swarm view (submit chat vs. open detail).
            pub(super) async fn dispatch_swarm_enter(
                app: &mut App,
                cwd: &Path,
                slot: &mut SessionSlot,
                registry: &Option<Arc<ProviderRegistry>>,
                worker_bridge: &Option<TuiWorkerBridge>,
                runtime: &TuiSessionHandle,
            ) {
                if app.state.input.trim().is_empty() {
                    app.state.swarm.enter_detail();
                } else {
                    $chat(app, cwd, slot, registry, worker_bridge, runtime).await;
                }
            }
        }
    };
}

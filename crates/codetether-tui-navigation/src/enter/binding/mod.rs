//! Frontend state and callbacks for view-level Enter routing.
mod asynchronous;
mod session;
mod swarm;
mod synchronous;
/// Declare the frontend Enter host and its session/detail handlers.
///
/// Synchronous view effects and asynchronous session operations stay separate;
/// the shared Enter dispatcher preserves their active-view selection.
/// # Examples
/// See the frontend Enter module for concrete types and callback paths.
#[macro_export]
macro_rules! declare_frontend_enter {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path, $mode:path;
     $file:path, $bus:path, $model:path, $subagents:path;
     $sessions:path, $settings:path, $chat:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $registry as ProviderRegistry;
        use $bridge as TuiWorkerBridge;
        use $runtime as TuiSessionHandle;
        use ::std::{path::Path, sync::Arc};
        $crate::__frontend_enter_session!($sessions, $settings);
        $crate::__frontend_enter_swarm!($chat);
        mod host {
            use super::{App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
                pub worker_bridge: &'a Option<TuiWorkerBridge>,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_enter_synchronous!($mode; $file, $bus, $model, $subagents);
            $crate::__frontend_enter_asynchronous!($chat);
        }
        /// Dispatch Enter to the handler matching the active view.
        pub(crate) async fn dispatch_enter(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::enter::dispatch(&mut host::Native {
                app,
                cwd,
                slot,
                registry,
                worker_bridge,
                runtime,
            })
            .await;
        }
    };
}

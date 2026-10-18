//! Frontend state selection for peer admission, dispatch and reply settlement.
mod host;
mod live;
mod presentation;
mod tick;

/// Declare frontend interactive-inbox bindings over existing session services.
///
/// Keeps notice presentation, live-turn settlement and prompt dispatch in
/// separate modules while preserving the frontend's concrete types.
/// # Examples
/// See the frontend inbox module for its selected types and dispatch callback.
#[macro_export]
macro_rules! declare_frontend_inbox {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path, $task:path;
     $message:path, $kind:path, $inbound:path; $dispatch:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $bridge as TuiWorkerBridge;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        use $task as IncomingTask;
        $crate::__frontend_inbox_live!($inbound);
        $crate::__frontend_inbox_host!($dispatch);
        $crate::__frontend_inbox_presentation!($message, $kind);
        pub(crate) async fn trigger_next(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::trigger_next(&mut host::Native {
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

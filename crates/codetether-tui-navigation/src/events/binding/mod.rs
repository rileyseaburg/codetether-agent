//! Frontend resources and callbacks for ordered key-event dispatch.
mod dispatch;
mod layers;
mod review;
/// Declare frontend modal, review and ordinary key dispatch adapters.
///
/// The shared dispatcher retains stage ordering and key admission policy;
/// the caller supplies existing frontend handlers and concrete resource types.
/// # Examples
/// See the frontend event-dispatch module for concrete bindings.
#[macro_export]
macro_rules! declare_frontend_key_dispatch {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path, $key:path;
     $priority:path, $modified:path, $unmodified:path;
     $question:path, $editor:path, $scroll:path, $okr:path;
     $review:path, $resume:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $bridge as TuiWorkerBridge;
        use $key as KeyEvent;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        mod host {
            use super::{
                App, Arc, Path, ProviderRegistry, SessionSlot, TuiSessionHandle, TuiWorkerBridge,
            };
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
                pub worker_bridge: &'a Option<TuiWorkerBridge>,
                pub runtime: &'a TuiSessionHandle,
            }
            $crate::__frontend_key_dispatch!($priority, $modified, $unmodified);
            $crate::__frontend_key_layers!($question, $editor, $scroll, $okr);
            $crate::__frontend_key_review!($review, $resume);
        }
        /// Handle one key event using shared admission and dispatch order.
        /// # Errors
        /// Returns errors from the selected review or ordinary key handler.
        pub(crate) async fn handle_event(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
            key: KeyEvent,
        ) -> anyhow::Result<bool> {
            $crate::events::dispatch(
                &mut host::Native {
                    app,
                    cwd,
                    slot,
                    registry,
                    worker_bridge,
                    runtime,
                },
                key,
            )
            .await
        }
    };
}

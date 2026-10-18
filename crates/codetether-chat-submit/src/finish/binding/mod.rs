//! Frontend finish context and typed entry point.
mod actions;
mod attachments;
/// Declare frontend prompt finish over existing state and service callbacks.
///
/// The shared policy retains operation ordering and passes through prompt,
/// attachment and provider values without cloning session ownership.
/// # Examples
/// See the frontend prompt finish module for concrete bindings.
#[macro_export]
macro_rules! declare_frontend_prompt_finish {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path,
     $image:path, $paste:path; $adopt:path, $publish:path, $dispatch:path, $expand:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $bridge as TuiWorkerBridge;
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

            $crate::__prompt_finish_actions!($adopt, $publish, $dispatch);
            $crate::__prompt_finish_attachments!($image, $paste, $expand);
        }
        /// Consume pending attachments and finalize a normally routed prompt.
        /// # Arguments
        /// Receives frontend/session context, provider configuration and prompt text.
        /// # Returns
        /// Unit after shared finalization completes.
        /// # Examples
        /// See the frontend submission host for invocation.
        pub(super) async fn finish_submit(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
            prompt: &str,
        ) {
            $crate::finish::finish(
                &mut host::Native {
                    app,
                    cwd,
                    slot,
                    registry,
                    worker_bridge,
                    runtime,
                },
                prompt,
            )
            .await;
        }
    };
}

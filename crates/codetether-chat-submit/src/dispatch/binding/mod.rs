//! Frontend dispatch context and typed entry point.
mod services;
/// Declare frontend prompt dispatch over existing state and service callbacks.
///
/// The shared policy retains operation ordering and passes through prompt,
/// attachment and provider values without cloning session ownership.
/// # Examples
/// See the frontend prompt dispatch module for concrete bindings.
#[macro_export]
macro_rules! declare_frontend_prompt_dispatch {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path,
     $image:path; $held:path, $started:path, $spawn:path, $missing:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $registry as ProviderRegistry;
        use $bridge as TuiWorkerBridge;
        use $runtime as TuiSessionHandle;
        use ::std::{path::Path, sync::Arc};
        use $image as ImageAttachment;
        mod host {
            use super::{App, Path, SessionSlot, TuiSessionHandle, TuiWorkerBridge};
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub worker_bridge: &'a Option<TuiWorkerBridge>,
                pub runtime: &'a TuiSessionHandle,
            }

            $crate::__prompt_dispatch_services!($image; $held, $started, $spawn, $missing);
        }
        /// Prepare the prompt and hand it to the configured provider runtime.
        /// # Arguments
        /// Receives frontend/session context, provider configuration, prompt and images.
        /// # Returns
        /// Unit after dispatch or presentation of a missing-provider status.
        /// # Examples
        /// See the frontend inbox and chat submission callers.
        pub(crate) async fn dispatch_prompt(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            prompt: &str,
            pending_images: Vec<ImageAttachment>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::dispatch::dispatch_prompt(
                &mut host::Native {
                    app,
                    cwd,
                    slot,
                    worker_bridge,
                    runtime,
                },
                registry.as_ref(),
                prompt,
                pending_images,
            )
            .await;
        }

    };
}

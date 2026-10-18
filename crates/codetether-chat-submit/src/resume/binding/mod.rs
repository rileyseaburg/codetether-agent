//! Frontend continuation state, runtime and session-picker bindings.
mod runtime;
mod selection;
mod source;
mod ui;
/// Declare continuation handlers over existing frontend and session resources.
///
/// Source, UI, runtime and picker effects are separate modules. Shared resume
/// policy still decides idle dispatch versus stalled-request cancellation.
/// # Examples
/// See the frontend continuation module for concrete types and callback paths.
#[macro_export]
macro_rules! declare_frontend_continuation {
    ($selection_visibility:vis,
     $app:path, $slot:path, $registry:path, $bridge:path, $runtime:path,
     $session:path, $provider_message:path, $chat_message:path, $notification:path;
     $prompt:path, $publish:path, $dispatch:path;
     $picker_trait:path, $picker_state:path, $load:path, $select:path $(,)?) => {
        use $app as App;
        use $slot as SessionSlot;
        use $registry as ProviderRegistry;
        use $bridge as TuiWorkerBridge;
        use $runtime as TuiSessionHandle;
        use ::std::{path::Path, sync::Arc};
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
            $crate::__frontend_continuation_source!($session, $provider_message, $prompt);
            $crate::__frontend_continuation_ui!($notification, $chat_message, $publish);
            $crate::__frontend_continuation_runtime!($dispatch);
            $crate::__frontend_continuation_selection!($picker_trait, $picker_state, $load);
        }
        /// Handle `/continue`. Returns `true` when `prompt` was the `/continue`
        /// command (and was handled); `false` otherwise so the caller keeps dispatching.
        pub(super) async fn run(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
            prompt: &str,
        ) -> bool {
            $crate::resume::run(
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
            .await
        }
        mod session_resume {
            use super::*;
            pub(crate) async fn resume_goal(
                app: &mut App,
                cwd: &Path,
                slot: &mut SessionSlot,
                registry: &Option<Arc<ProviderRegistry>>,
                worker_bridge: &Option<TuiWorkerBridge>,
                runtime: &TuiSessionHandle,
            ) -> bool {
                $crate::resume::automatic::run(
                    &mut host::Native {
                        app,
                        cwd,
                        slot,
                        registry,
                        worker_bridge,
                        runtime,
                    },
                    registry.is_some(),
                )
                .await
            }
            $selection_visibility async fn select_session(
                app: &mut App,
                cwd: &Path,
                slot: &mut SessionSlot,
                registry: &Option<Arc<ProviderRegistry>>,
                worker_bridge: &Option<TuiWorkerBridge>,
                runtime: &TuiSessionHandle,
            ) {
                $select(&mut host::Native {
                    app,
                    cwd,
                    slot,
                    registry,
                    worker_bridge,
                    runtime,
                })
                .await;
            }
        }
        pub(crate) use session_resume::resume_goal;
        pub(super) use session_resume::select_session;
    };
}

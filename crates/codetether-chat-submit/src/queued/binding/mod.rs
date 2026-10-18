//! Frontend state and service selection for queued chat submission.
mod commands;
mod input;
mod queue;

/// Declare queued-prompt delivery over the frontend's existing state and services.
///
/// Keeps command/answer delivery, queue gating and input submission in separate
/// modules while the shared [crate::queued::drain] owns their execution order.
///
/// # Examples
/// See the frontend queued-prompt module for concrete type and callback paths.
#[macro_export]
macro_rules! declare_frontend_queued_prompt {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path;
     $commands:path, $answers:path, $enter:path;
     $pending:path, $held:path, $take:path $(,)?) => {
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
            $crate::__queued_prompt_commands!($commands, $answers);
            $crate::__queued_prompt_input!($enter);
            $crate::__queued_prompt_queue!($pending, $held, $take);
        }
        pub(super) async fn drain(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::queued::drain(&mut host::Native {
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

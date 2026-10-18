//! Frontend entry points for deferred continuation and restart.
/// Implementation detail of [crate::declare_frontend_goal_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_resume_entries {
    () => {
        mod resume {
            use super::*;
            pub(crate) async fn drain(
                app: &mut App,
                cwd: &Path,
                slot: &mut SessionSlot,
                registry: &Option<Arc<ProviderRegistry>>,
                bridge: &Option<TuiWorkerBridge>,
                runtime: &TuiSessionHandle,
            ) {
                $crate::input::drain(&mut super::resume_host::Native {
                    app,
                    cwd,
                    slot,
                    registry,
                    bridge,
                    runtime,
                })
                .await;
            }
            pub(crate) fn restart(app: &mut App, id: &str, runtime: &TuiSessionHandle) {
                $crate::input::restart(&mut super::restart_host::Native { app, runtime }, id);
            }
        }
    };
}

//! Admit parked peer work before selecting the next queued task.
/// Declare the frontend inbox admission/dispatch step used by its event loop.
/// # Examples
/// See the event-loop inbox module for its concrete type and callback selections.
#[macro_export]
macro_rules! declare_frontend_inbox_tick {
    ($app:path, $slot:path, $registry:path, $bridge:path, $runtime:path;
     $admit:path, $trigger:path $(,)?) => {
        use ::std::sync::Arc;
        use $app as App;
        use $bridge as TuiWorkerBridge;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        pub async fn trigger_next(
            app: &mut App,
            cwd: &std::path::Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
        ) {
            $admit(app);
            $trigger(app, cwd, slot, registry, worker_bridge, runtime).await;
        }
    };
}

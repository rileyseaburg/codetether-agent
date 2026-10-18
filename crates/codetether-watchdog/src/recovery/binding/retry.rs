//! Frontend entry point for pending watchdog recovery.
/// Declare watchdog retry execution over frontend session/runtime types.
///
/// Request rejection restores the session to its original slot.
/// # Examples
/// ```text
/// codetether_watchdog::declare_frontend_watchdog_retry!(
///     App, Slot, Registry, Runtime, Session, Request,
/// );
/// ```
#[macro_export]
macro_rules! declare_frontend_watchdog_retry {
    ($app:path, $slot:path, $registry:path, $runtime:path, $session:path, $request:path $(,)?) => {
        use ::std::sync::Arc;
        use $app as App;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        $crate::__frontend_watchdog_retry_host!(
            $app, $slot, $registry, $runtime, $session, $request
        );
        pub(super) async fn execute(
            app: &mut App,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            runtime: &TuiSessionHandle,
        ) {
            $crate::recovery::execute(&mut host::Native { app, slot, runtime }, registry.as_ref())
                .await;
        }
    };
}

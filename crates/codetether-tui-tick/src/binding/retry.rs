//! Per-tick watchdog and delayed model retry ordering.
/// Declare the frontend retry step after a UI tick.
///
/// The watchdog callback completes before executing a scheduled smart switch.
/// # Examples
/// See the frontend tick retry module for selected resources and callbacks.
#[macro_export]
macro_rules! declare_frontend_tick_retry {
    ($app:ty, $slot:ty, $registry:ty, $runtime:ty;
     $watchdog:path, $smart_retry:path $(,)?) => {
        /// Check for stalled work, then execute a pending model retry.
        pub(crate) async fn check_and_retry(
            app: &mut $app,
            slot: &mut $slot,
            registry: &Option<::std::sync::Arc<$registry>>,
            runtime: &$runtime,
            interval: ::std::time::Duration,
        ) {
            $watchdog(app, runtime, interval).await;
            $smart_retry(app, slot, registry, runtime).await;
        }
    };
}

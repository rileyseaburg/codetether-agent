//! Timer and per-tick watchdog checks with unchanged cancellation gates.
/// Declare frontend watchdog UI access and cancellation checks.
///
/// Timer checks require a retained in-flight prompt. Per-tick checks keep the
/// ungated detection path. Both delegate feedback and cancellation ordering to
/// the shared recovery policy.
/// # Examples
/// ```text
/// codetether_watchdog::declare_frontend_watchdog!(App, Runtime, TIMEOUT, detect);
/// ```
#[macro_export]
macro_rules! declare_frontend_watchdog {
    ($app:path, $runtime:path, $timeout:path, $detect:path $(,)?) => {
        use ::std::time::Duration;
        use $app as App;
        use $runtime as TuiSessionHandle;
        use $timeout as MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS;
        $crate::__frontend_watchdog_ui!($app);
        impl $crate::recovery::CheckHost for App {
            fn detect(&self, interval: Duration) -> Option<$crate::WatchdogNotification> {
                $detect(&self.state, interval)
            }
            fn has_inflight(&self) -> bool {
                self.state.main_inflight_prompt.is_some()
            }
        }
        /// Check the timer watchdog and cancel a stalled prompt eligible for retry.
        pub(super) async fn maybe_watchdog_restart(
            app: &mut App,
            runtime: &TuiSessionHandle,
            watchdog_interval: Duration,
        ) {
            $crate::recovery::check(
                app,
                watchdog_interval,
                MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
                true,
                || runtime.cancel_current(),
            )
            .await;
        }

        pub(super) async fn check_tick(
            app: &mut App,
            runtime: &TuiSessionHandle,
            interval: std::time::Duration,
        ) {
            $crate::recovery::check(
                app,
                interval,
                MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
                false,
                || runtime.cancel_current(),
            )
            .await;
        }
    };
}

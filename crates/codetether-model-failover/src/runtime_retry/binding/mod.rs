//! Frontend state and owned-session transfer for scheduled model retries.
mod host;
mod submission;

/// Declare scheduled-retry entry, state access and runtime submission.
///
/// Frontend types retain session persistence, timing updates and request
/// ownership. Rejected runtime submissions restore their session to the slot.
///
/// # Examples
/// ```text
/// codetether_model_failover::declare_frontend_retry!(
///     App, SessionSlot, ProviderRegistry, Runtime, PromptRequest,
/// );
/// ```
#[macro_export]
macro_rules! declare_frontend_retry {
    ($app:path, $slot:path, $registry:path, $runtime:path, $request:path $(,)?) => {
        $crate::__frontend_retry_host!($app, $slot, $registry, $runtime);
        $crate::__frontend_retry_submission!($slot, $registry, $runtime, $request);
        /// Execute a pending scheduled retry with the configured provider registry.
        pub(super) async fn execute_smart_switch_retry(
            app: &mut $app,
            slot: &mut $slot,
            registry: &Option<::std::sync::Arc<$registry>>,
            runtime: &$runtime,
        ) {
            $crate::runtime_retry::execute(
                &mut host::Native { app, slot, runtime },
                registry.as_ref(),
            )
            .await;
        }
    };
}

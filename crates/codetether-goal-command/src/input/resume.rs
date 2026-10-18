//! Deferred cancellation and resume after session ownership returns.
use std::future::Future;
/// Native cancellation and stale UI state removal.
/// # Implementors
/// Forward cancellation and clear only the existing watchdog/in-flight fields.
pub trait Restart {
    /// Cancel the current execution.
    fn cancel(&mut self);
    /// Clear watchdog notification.
    fn clear_watchdog(&mut self);
    /// Clear the in-flight prompt preview.
    fn clear_inflight(&mut self);
}
/// Native ownership checks and continuation execution.
/// # Implementors
/// Keep checks observational and resume through the existing auto-start path.
pub trait Resume {
    /// Whether an execution is in progress.
    fn processing(&self) -> bool;
    /// Whether the session is available for borrowing.
    fn available(&self) -> bool;
    /// Whether providers are installed.
    fn providers(&self) -> bool;
    /// Current session identity.
    fn session_id(&self) -> String;
    /// Resume using the freshly persisted goal.
    fn resume(&mut self) -> impl Future<Output = ()>;
}
/// Record a continuation intent, cancel execution and clear stale UI state.
/// # Arguments
/// Receives native restart actions and a session ID.
/// # Returns
/// Completes after marking the one-shot pending intent and cancellation.
/// # Examples
/// ```no_run
/// fn schedule<H: codetether_goal_command::input::Restart>(host: &mut H) {
///     codetether_goal_command::input::restart(host, "session");
/// }
/// ```
pub fn restart<H: Restart>(app: &mut H, id: &str) {
    super::pending::mark(id);
    app.cancel();
    app.clear_watchdog();
    app.clear_inflight();
}
/// Consume pending continuation only when the session and providers are available.
/// # Arguments
/// `app` supplies current ownership state and the native resume operation.
/// # Returns
/// Completes without consuming the intent while blocked on ownership/providers.
/// # Examples
/// ```no_run
/// async fn resume<H: codetether_goal_command::input::Resume>(host: &mut H) {
///     codetether_goal_command::input::drain(host).await;
/// }
/// ```
pub async fn drain<H: Resume>(app: &mut H) {
    if app.processing() || !app.available() || !app.providers() {
        return;
    }
    if super::pending::take(&app.session_id()) {
        app.resume().await;
    }
}

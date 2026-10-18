//! Manual cancellation and dismissal of a watchdog notification.
use super::Ui;
/// Stop processing and clear stream/timing state before dismissing the alert.
/// # Arguments
/// Receives existing recovery state and native stream/timing effects.
/// # Returns
/// Updates cancellation status without initiating an automatic retry.
/// # Examples
/// ```no_run
/// fn cancel<U: codetether_watchdog::recovery::Ui>(ui: &mut U) {
///     codetether_watchdog::recovery::cancel(ui);
/// }
/// ```
pub fn cancel<H: Ui>(host: &mut H) {
    *host.state().processing = false;
    host.clear_streaming();
    host.clear_timing();
    *host.state().notification = None;
    *host.state().status = "Request cancelled.".to_string();
}
/// Dismiss the alert after the event loop has handled restart.
/// # Arguments
/// Receives the existing recovery state.
/// # Returns
/// Clears only the notification.
/// # Examples
/// ```no_run
/// fn dismiss<U: codetether_watchdog::recovery::Ui>(ui: &mut U) {
///     codetether_watchdog::recovery::dismiss(ui);
/// }
/// ```
pub fn dismiss<H: Ui>(host: &mut H) {
    *host.state().notification = None;
}

//! User-facing reconnect progress messages.
use crate::MAX_RECONNECT_ATTEMPTS;
/// Build the chat transcript reconnect banner.
///
/// # Arguments
/// * `attempt` — Current reconnect count.
///
/// # Returns
/// A progress banner including the fixed retry budget.
///
/// # Examples
/// ```
/// assert!(codetether_stream_recovery::reconnect_banner(1).contains("1/3"));
/// ```
pub fn reconnect_banner(attempt: u32) -> String {
    format!("⚡ Stream disconnected — reconnecting (attempt {attempt}/{MAX_RECONNECT_ATTEMPTS})…")
}
/// Build the status-line reconnect message.
///
/// # Arguments
/// * `attempt` — Current reconnect count.
///
/// # Returns
/// A short reconnect status string.
///
/// # Examples
/// ```
/// assert_eq!(codetether_stream_recovery::reconnect_status(1), "Reconnecting (1/3)…");
/// ```
pub fn reconnect_status(attempt: u32) -> String {
    format!("Reconnecting ({attempt}/{MAX_RECONNECT_ATTEMPTS})…")
}

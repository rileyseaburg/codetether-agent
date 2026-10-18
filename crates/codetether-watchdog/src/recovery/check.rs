//! Ordered stall detection, recovery feedback, and executor cancellation.
use super::Ui;
use crate::WatchdogNotification;
use std::{future::Future, time::Duration};

/// Exposes stall detection and the presence of an in-flight prompt.
/// # Implementors
/// Frontends bind their existing detector and recovery state fields.
pub trait CheckHost: Ui {
    /// Detect a stall using the current frontend state and timeout interval.
    fn detect(&self, interval: Duration) -> Option<WatchdogNotification>;
    /// Whether the frontend has retained the prompt needed for a retry.
    fn has_inflight(&self) -> bool;
}

/// Detect a stall, apply recovery feedback, then cancel its executor.
/// # Arguments
/// * `host` — Existing detector and mutable recovery UI.
/// * `interval` — Stall interval supplied to detection.
/// * `timeout` — Timeout in seconds displayed by recovery feedback.
/// * `require_inflight` — Require a retained prompt before recovery; timer
///   checks enable this gate, while per-tick checks preserve their ungated path.
/// * `cancel` — Lazily creates the cancellation future after feedback is applied.
/// # Returns
/// Completes after cancellation, or immediately when no eligible stall exists.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_watchdog::recovery::CheckHost) {
/// codetether_watchdog::recovery::check(
///     host, std::time::Duration::from_secs(60), 60, true, || async {},
/// ).await;
/// # }
/// ```
pub async fn check<H, F, D>(
    host: &mut H,
    interval: Duration,
    timeout: u64,
    require_inflight: bool,
    cancel: F,
) where
    H: CheckHost,
    F: FnOnce() -> D,
    D: Future<Output = ()>,
{
    let Some(notification) = host.detect(interval) else {
        return;
    };
    if require_inflight && !host.has_inflight() {
        return;
    }
    super::apply(host, notification, timeout);
    cancel().await;
}

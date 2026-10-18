//! Event-loop refresh and watchdog intervals.
use std::time::Duration;

/// How often the watchdog timer fires to check for stalls.
const WATCHDOG_CHECK_INTERVAL: Duration = Duration::from_secs(2);

/// UI, audit and watchdog timer intervals.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let timers = codetether_tui_reactor::Timers::new(std::time::Duration::from_secs(60));
/// assert_eq!(timers.watchdog_interval.as_secs(), 60);
/// # }
/// ```
pub struct Timers {
    /// General UI refresh cadence.
    pub tick: tokio::time::Interval,
    /// Audit refresh cadence.
    pub audit: tokio::time::Interval,
    /// Watchdog polling cadence.
    pub watchdog: tokio::time::Interval,
    /// Requested inactivity timeout, independent of polling cadence.
    pub watchdog_interval: Duration,
}

impl Timers {
    /// Start UI (100 ms), audit (1 s) and watchdog (2 s) intervals.
    /// # Arguments
    /// * `watchdog_interval` - Inactivity threshold consumed by watchdog handling.
    /// # Returns
    /// Running intervals and the unchanged inactivity threshold.
    /// # Panics
    /// Panics when called outside a Tokio runtime with time enabled.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let _timers = codetether_tui_reactor::Timers::new(std::time::Duration::from_secs(60));
    /// # }
    /// ```
    pub fn new(watchdog_interval: Duration) -> Self {
        Self {
            tick: tokio::time::interval(Duration::from_millis(100)),
            audit: tokio::time::interval(Duration::from_secs(1)),
            // The watchdog fires frequently (every 2s) but the *timeout*
            // (watchdog_interval) determines how long of inactivity is
            // required before action is taken. This decouples "how often we
            // check" from "how long we tolerate silence."
            watchdog: tokio::time::interval(WATCHDOG_CHECK_INTERVAL),
            watchdog_interval,
        }
    }
}

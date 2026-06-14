use std::time::Duration;

/// How often the watchdog timer fires to check for stalls.
const WATCHDOG_CHECK_INTERVAL: Duration = Duration::from_secs(2);

pub(crate) struct LoopTimers {
    pub tick: tokio::time::Interval,
    pub watchdog: tokio::time::Interval,
    pub watchdog_interval: Duration,
}

impl LoopTimers {
    pub fn new(watchdog_interval: Duration) -> Self {
        Self {
            tick: tokio::time::interval(Duration::from_millis(100)),
            // The watchdog fires frequently (every 2s) but the *timeout*
            // (watchdog_interval) determines how long of inactivity is
            // required before action is taken. This decouples "how often we
            // check" from "how long we tolerate silence."
            watchdog: tokio::time::interval(WATCHDOG_CHECK_INTERVAL),
            watchdog_interval,
        }
    }
}

use std::time::Duration;

pub(crate) struct LoopTimers {
    pub tick: tokio::time::Interval,
    pub watchdog: tokio::time::Interval,
    pub watchdog_interval: Duration,
}

impl LoopTimers {
    pub fn new(watchdog_interval: Duration) -> Self {
        Self {
            tick: tokio::time::interval(Duration::from_millis(50)),
            watchdog: tokio::time::interval(watchdog_interval),
            watchdog_interval,
        }
    }
}

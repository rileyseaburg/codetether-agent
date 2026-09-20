//! Exponential backoff for bounded mux polling loops.
//!
//! Startup, shutdown, and runtime-registration waits all poll for a state
//! change. A fixed 50 ms tick meant a server that published its record in 1 ms
//! still cost ~50 ms of latency. This starts at 250 us and doubles to a 50 ms
//! ceiling, preserving each caller's original total budget for genuinely slow
//! transitions while making the common fast path essentially free.

use std::time::Duration;

const FIRST: Duration = Duration::from_micros(250);
const CEILING: Duration = Duration::from_millis(50);
const DEFAULT_BUDGET: Duration = Duration::from_secs(5);

/// Bounded doubling sleep schedule.
pub(in crate::mux) struct Backoff {
    delay: Duration,
    elapsed: Duration,
    budget: Duration,
}

impl Backoff {
    /// Backoff with the default 5 second total budget.
    pub(in crate::mux) fn new() -> Self {
        Self::with_budget(DEFAULT_BUDGET)
    }

    /// Backoff bounded by an explicit total wait budget.
    pub(in crate::mux) fn with_budget(budget: Duration) -> Self {
        Self {
            delay: FIRST,
            elapsed: Duration::ZERO,
            budget,
        }
    }

    /// Whether the total wait budget still allows another attempt.
    pub(in crate::mux) fn remaining(&self) -> bool {
        self.elapsed < self.budget
    }

    /// Wait for the current interval, then double it up to the ceiling.
    pub(in crate::mux) async fn sleep(&mut self) {
        tokio::time::sleep(self.delay).await;
        self.elapsed += self.delay;
        self.delay = (self.delay * 2).min(CEILING);
    }
}

#[cfg(test)]
#[path = "backoff_tests.rs"]
mod tests;

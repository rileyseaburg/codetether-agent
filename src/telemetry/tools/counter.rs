//! Lock-free success/failure counter for tool invocations.

use std::sync::atomic::{AtomicU64, Ordering};

/// Cumulative `(total, failures)` counter shared across the whole process.
///
/// Prefer the [`super::super::TOOL_EXECUTIONS`] singleton — constructing your
/// own instance is only useful in tests.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::AtomicToolCounter;
///
/// let c = AtomicToolCounter::new();
/// c.record(true);
/// c.record(false);
/// c.record(true);
///
/// let (total, failures) = c.get();
/// assert_eq!((total, failures), (3, 1));
/// ```
#[derive(Debug)]
pub struct AtomicToolCounter {
    count: AtomicU64,
    failures: AtomicU64,
}

impl AtomicToolCounter {
    /// Construct a zeroed counter.
    pub fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
            failures: AtomicU64::new(0),
        }
    }

    /// Record a single invocation. Increments `failures` iff `success` is false.
    pub fn record(&self, success: bool) {
        self.count.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.failures.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Load `(total_invocations, total_failures)` with `Relaxed` ordering.
    pub fn get(&self) -> (u64, u64) {
        (
            self.count.load(Ordering::Relaxed),
            self.failures.load(Ordering::Relaxed),
        )
    }
}

impl Default for AtomicToolCounter {
    fn default() -> Self {
        Self::new()
    }
}

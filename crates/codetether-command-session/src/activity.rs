//! Output-liveness accounting for a running command session.

use tokio::time::{Duration, Instant};

/// Tracks when a session last produced bytes so silent polls stay observable.
pub(crate) struct Activity {
    started: Instant,
    last_output: Instant,
    total_bytes: usize,
}

impl Activity {
    pub(crate) fn new() -> Self {
        let now = Instant::now();
        Self {
            started: now,
            last_output: now,
            total_bytes: 0,
        }
    }

    /// Records bytes observed during one poll; zero bytes extends the silence.
    pub(crate) fn record(&mut self, bytes: usize) {
        if bytes > 0 {
            self.last_output = Instant::now();
            self.total_bytes = self.total_bytes.saturating_add(bytes);
        }
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    pub(crate) fn silent(&self) -> Duration {
        self.last_output.elapsed()
    }

    pub(crate) fn total_bytes(&self) -> usize {
        self.total_bytes
    }
}

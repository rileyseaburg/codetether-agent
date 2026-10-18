//! Discard requests outside the rolling window.
use super::{Instant, SlidingWindow};
impl SlidingWindow {
    /// Drop entries older than 60 seconds.
    pub(super) fn evict(&mut self) {
        let cutoff = Instant::now() - Self::WINDOW;
        while self.entries.front().is_some_and(|e| e.at < cutoff) {
            self.entries.pop_front();
        }
    }
}

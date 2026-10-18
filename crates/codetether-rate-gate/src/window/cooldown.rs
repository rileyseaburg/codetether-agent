//! Estimate time until the oldest recorded request expires.
use super::SlidingWindow;
impl SlidingWindow {
    /// Estimated ms until the oldest entry exits the window.
    /// Returns 0 if the window is empty.
    pub(crate) fn ms_until_oldest_expires(&mut self) -> u64 {
        self.evict();
        self.entries
            .front()
            .map(|e| {
                let age = e.at.elapsed();
                Self::WINDOW.saturating_sub(age).as_millis() as u64
            })
            .unwrap_or(0)
    }
}

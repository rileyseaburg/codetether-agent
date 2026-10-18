//! Read request/token totals and record completed usage.
use super::{Entry, Instant, SlidingWindow};
impl SlidingWindow {
    /// Current request count in the last 60 s.
    pub(crate) fn rpm(&mut self) -> u32 {
        self.evict();
        self.entries.len() as u32
    }

    /// Current token count in the last 60 s.
    pub(crate) fn tpm(&mut self) -> u32 {
        self.evict();
        self.entries.iter().map(|e| e.tokens).sum()
    }

    /// Record a completed request with `tokens` output tokens.
    pub(crate) fn record(&mut self, tokens: u32) {
        self.evict();
        self.entries.push_back(Entry {
            at: Instant::now(),
            tokens,
        });
    }
}

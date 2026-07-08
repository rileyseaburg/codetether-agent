//! Sliding 60-second window tracking RPM and TPM for one model.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// One recorded request in the sliding window.
#[derive(Debug)]
struct Entry {
    at: Instant,
    tokens: u32,
}

/// Sliding 60-second window of requests for a single (provider, model) key.
#[derive(Debug, Default)]
pub(super) struct SlidingWindow {
    entries: VecDeque<Entry>,
}

impl SlidingWindow {
    const WINDOW: Duration = Duration::from_secs(60);

    /// Drop entries older than 60 seconds.
    fn evict(&mut self) {
        let cutoff = Instant::now() - Self::WINDOW;
        while self.entries.front().is_some_and(|e| e.at < cutoff) {
            self.entries.pop_front();
        }
    }

    /// Current request count in the last 60 s.
    pub(super) fn rpm(&mut self) -> u32 {
        self.evict();
        self.entries.len() as u32
    }

    /// Current token count in the last 60 s.
    pub(super) fn tpm(&mut self) -> u32 {
        self.evict();
        self.entries.iter().map(|e| e.tokens).sum()
    }

    /// Record a completed request with `tokens` output tokens.
    pub(super) fn record(&mut self, tokens: u32) {
        self.evict();
        self.entries.push_back(Entry {
            at: Instant::now(),
            tokens,
        });
    }

    /// Estimated ms until the oldest entry exits the window.
    /// Returns 0 if the window is empty.
    pub(super) fn ms_until_oldest_expires(&mut self) -> u64 {
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

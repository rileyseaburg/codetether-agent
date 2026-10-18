//! Rolling-window storage for a single provider/model pair.
use std::collections::VecDeque;
use std::time::{Duration, Instant};
mod accounting;
mod cooldown;
mod eviction;

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
}

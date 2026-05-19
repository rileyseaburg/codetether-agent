//! Rolling chat-turn latency statistics.
//!
//! Tracks end-to-end wall-clock time from Enter keypress to
//! `SessionEvent::Done` for every completed user turn.

const WINDOW: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TurnSample {
    e2e_ms: u64,
    ttft_ms: Option<u64>,
}

/// Rolling window of chat-turn latency samples.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::app::state::chat_latency::ChatLatencyStats;
/// let mut stats = ChatLatencyStats::default();
/// stats.record(1200, Some(180));
/// stats.record(3400, Some(220));
/// assert_eq!(stats.count(), 2);
/// assert_eq!(stats.last_e2e_ms(), Some(3400));
/// assert_eq!(stats.avg_e2e_ms(), Some(2300));
/// ```
#[derive(Debug, Clone, Default)]
pub struct ChatLatencyStats {
    samples: Vec<TurnSample>,
}

impl ChatLatencyStats {
    /// Record a completed turn.
    pub fn record(&mut self, e2e_ms: u64, ttft_ms: Option<u64>) {
        if self.samples.len() >= WINDOW {
            self.samples.remove(0);
        }
        self.samples.push(TurnSample { e2e_ms, ttft_ms });
    }

    pub fn count(&self) -> usize {
        self.samples.len()
    }

    pub fn last_e2e_ms(&self) -> Option<u64> {
        self.samples.last().map(|s| s.e2e_ms)
    }

    pub fn avg_e2e_ms(&self) -> Option<u64> {
        (!self.samples.is_empty())
            .then(|| self.samples.iter().map(|s| s.e2e_ms).sum::<u64>() / self.samples.len() as u64)
    }

    pub fn min_e2e_ms(&self) -> Option<u64> {
        self.samples.iter().map(|s| s.e2e_ms).min()
    }

    pub fn max_e2e_ms(&self) -> Option<u64> {
        self.samples.iter().map(|s| s.e2e_ms).max()
    }

    pub fn p95_e2e_ms(&self) -> Option<u64> {
        if self.samples.len() < 3 {
            return self.max_e2e_ms();
        }
        let mut v: Vec<u64> = self.samples.iter().map(|s| s.e2e_ms).collect();
        v.sort_unstable();
        let i = (v.len() as f64 * 0.95).ceil() as usize - 1;
        Some(v[i.min(v.len() - 1)])
    }

    pub fn avg_ttft_ms(&self) -> Option<u64> {
        let t: Vec<u64> = self.samples.iter().filter_map(|s| s.ttft_ms).collect();
        (!t.is_empty()).then(|| t.iter().sum::<u64>() / t.len() as u64)
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

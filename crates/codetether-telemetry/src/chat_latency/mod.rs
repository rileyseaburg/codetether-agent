//! Rolling timing statistics for completed user turns.
//!
//! [ChatLatencyStats] retains the most recent 100 end-to-end and optional
//! first-token timings. Record completed turns, then query recent, average,
//! extrema or percentile timings without depending on a UI or clock source.
mod percentile;
mod record;
mod sample;
mod summary;
use sample::TurnSample;

const WINDOW: usize = 100;

/// Rolling window of chat-turn latency samples.
///
/// # Examples
///
/// ```rust
/// use codetether_telemetry::chat_latency::ChatLatencyStats;
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

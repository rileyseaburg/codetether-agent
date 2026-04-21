//! Lock-light atomic counter for LLM token usage.
//!
//! Global and per-model counters live here. Per-model recording uses a
//! `try_lock` fast path so that a contended counter never blocks the
//! completion path — at worst, the stats are briefly stale.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;

use super::snapshot::{GlobalTokenSnapshot, TokenUsageSnapshot};
use super::totals::TokenTotals;

/// Process-wide token counter. Cheap `record` on the hot path; richer
/// per-model stats guarded by an async-compatible mutex and accessed
/// best-effort via `try_lock`.
///
/// Prefer the [`super::super::TOKEN_USAGE`] singleton — constructing your
/// own instance is only useful in tests.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::AtomicTokenCounter;
///
/// let c = AtomicTokenCounter::new();
/// c.record(100, 50);
/// let (p, o, t) = c.get();
/// assert_eq!((p, o, t), (100, 50, 150));
/// ```
#[derive(Debug)]
pub struct AtomicTokenCounter {
    pub(super) prompt_tokens: AtomicU64,
    pub(super) completion_tokens: AtomicU64,
    pub(super) total_tokens: AtomicU64,
    pub(super) request_count: AtomicU64,
    pub(super) model_usage: Mutex<HashMap<String, (u64, u64)>>,
    /// Per-model prompt-cache stats: `(cache_read_tokens, cache_write_tokens)`.
    /// Tracked separately so [`crate::provider::pricing`] can apply the
    /// discounted / surcharged rates that Anthropic / Bedrock use.
    pub(super) model_cache_usage: Mutex<HashMap<String, (u64, u64)>>,
    /// Per-model **last turn** prompt token count. Used by the TUI to show
    /// how close the current in-flight conversation is to the model's
    /// context window, independent of cumulative lifetime usage.
    pub(super) model_last_prompt_tokens: Mutex<HashMap<String, u64>>,
}

impl AtomicTokenCounter {
    /// Construct a zeroed counter.
    pub fn new() -> Self {
        Self {
            prompt_tokens: AtomicU64::new(0),
            completion_tokens: AtomicU64::new(0),
            total_tokens: AtomicU64::new(0),
            request_count: AtomicU64::new(0),
            model_usage: Mutex::new(HashMap::new()),
            model_cache_usage: Mutex::new(HashMap::new()),
            model_last_prompt_tokens: Mutex::new(HashMap::new()),
        }
    }

    /// Record a single completion. Increments all four global atomics
    /// relaxed-ordering — correctness only requires eventual visibility.
    pub fn record(&self, prompt: u64, completion: u64) {
        self.prompt_tokens.fetch_add(prompt, Ordering::Relaxed);
        self.completion_tokens
            .fetch_add(completion, Ordering::Relaxed);
        self.total_tokens
            .fetch_add(prompt + completion, Ordering::Relaxed);
        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Return the global `(prompt, completion, total)` triple.
    pub fn get(&self) -> (u64, u64, u64) {
        (
            self.prompt_tokens.load(Ordering::Relaxed),
            self.completion_tokens.load(Ordering::Relaxed),
            self.total_tokens.load(Ordering::Relaxed),
        )
    }

    /// Snapshot the global counters into a plain-data struct.
    pub fn global_snapshot(&self) -> GlobalTokenSnapshot {
        let (prompt, completion, total) = self.get();
        let mut snapshot = GlobalTokenSnapshot::new(prompt, completion, total);
        snapshot.request_count = self.request_count.load(Ordering::Relaxed);
        snapshot
    }

    /// Snapshot every tracked model's usage. Returns an empty `Vec` if the
    /// per-model map is currently contended.
    pub fn model_snapshots(&self) -> Vec<TokenUsageSnapshot> {
        let Ok(usage) = self.model_usage.try_lock() else {
            return Vec::new();
        };
        usage
            .iter()
            .map(|(name, (input, output))| TokenUsageSnapshot {
                name: name.clone(),
                prompt_tokens: *input,
                completion_tokens: *output,
                total_tokens: input + output,
                totals: TokenTotals::new(*input, *output),
                timestamp: Utc::now(),
                request_count: 0,
            })
            .collect()
    }
}

impl Default for AtomicTokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

mod record_model;

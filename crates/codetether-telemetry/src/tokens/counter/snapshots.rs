//! Focused operations for the parent telemetry type.

use super::*;

impl AtomicTokenCounter {
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

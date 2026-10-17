//! Persisted derivation policy selectors.
use serde::{Deserialize, Serialize};

/// Per-session derivation strategy selector.
///
/// Defaults to [`DerivePolicy::Incremental`] so RLM prepares relevant
/// context continuously instead of waiting for compaction pressure.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "policy", rename_all = "snake_case")]
pub enum DerivePolicy {
    /// Phase A clone + experimental + `enforce_on_messages` + pairing.
    /// The historical pipeline.
    Legacy,
    /// Lu et al. (arXiv:2510.06727) reset-to-(prompt, summary).
    ///
    /// When the request's estimated token cost exceeds
    /// `threshold_tokens`, replace everything older than the last user
    /// turn with a single RLM-generated summary. The derived context for
    /// the next provider call contains `[summary, active_task_tail...]`
    /// so short continuation turns do not drop the task-defining prompt.
    Reset {
        /// Token budget that triggers the reset. Typically ~95 % of
        /// the model's working context length.
        threshold_tokens: usize,
    },
    /// Liu et al. (arXiv:2512.22087) incremental select-then-pack
    /// derivation.
    ///
    /// Score every entry against the latest user turn's
    /// [`RelevanceMeta`](crate::relevance::RelevanceMeta) and
    /// greedy-pack the highest-scoring set into `budget_tokens`. The
    /// recent window is always retained; older entries are kept only
    /// when their relevance to the active task is high.
    Incremental {
        /// Per-call working-context budget. Older entries are dropped
        /// once the selected set would exceed this estimate.
        budget_tokens: usize,
    },
    /// ClawVM replay oracle (Phase B step 22). Evaluation-only.
    ///
    /// Given a recorded trace, picks representations with `h`-turn
    /// future-demand lookahead to minimise fault count. Not suitable
    /// for production — only for benchmarking online vs oracle gap.
    OracleReplay {
        /// Lookahead horizon in turns. ClawVM uses h=3.
        lookahead: usize,
    },
}

impl Default for DerivePolicy {
    fn default() -> Self {
        DerivePolicy::Incremental { budget_tokens: 0 }
    }
}

impl DerivePolicy {
    /// Human-readable short name for logs, journal entries, and
    /// telemetry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_context_policy::derivation::DerivePolicy;
    ///
    /// assert_eq!(DerivePolicy::Legacy.kind(), "legacy");
    /// assert_eq!(
    ///     DerivePolicy::Reset { threshold_tokens: 0 }.kind(),
    ///     "reset",
    /// );
    /// ```
    pub fn kind(&self) -> &'static str {
        match self {
            DerivePolicy::Legacy => "legacy",
            DerivePolicy::Reset { .. } => "reset",
            DerivePolicy::Incremental { .. } => "incremental",
            DerivePolicy::OracleReplay { .. } => "oracle_replay",
        }
    }
}
#[cfg(test)]
mod tests;

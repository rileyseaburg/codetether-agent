//! RLM progress and completion payloads for [`SessionEvent`].
//!
//! The [`RlmProgressEvent`] type is the **bus-facing firewall** around the
//! internal `ProcessProgress` struct in [`crate::rlm::router`]: only the
//! fields that are safe to broadcast and persist are copied across. If
//! `ProcessProgress` ever grows a sensitive field (auth token, absolute
//! local path, raw prompt), the durable sink will not leak it.
//!
//! [`SessionEvent`]: crate::session::SessionEvent

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One progress tick emitted by an in-flight RLM analysis loop.
///
/// Emitted at iteration boundaries so the TUI can render a spinner with
/// `iteration / max_iterations` and a short status string. This type is
/// constructed inside the session crate from the private `ProcessProgress`
/// — do **not** add fields that should not appear on the bus or in
/// persistent traces.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::RlmProgressEvent;
/// use uuid::Uuid;
///
/// let p = RlmProgressEvent {
///     trace_id: Uuid::nil(),
///     iteration: 3,
///     max_iterations: 15,
///     status: "grepping".into(),
/// };
/// assert!(p.fraction() > 0.0);
/// assert!(p.fraction() < 1.0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmProgressEvent {
    /// Correlates every event from a single RLM invocation, including the
    /// terminal [`RlmCompletion`].
    pub trace_id: Uuid,
    /// 1-based iteration number currently executing.
    pub iteration: usize,
    /// Maximum iterations permitted by config.
    pub max_iterations: usize,
    /// Short human-readable status ("grepping", "summarising", etc.).
    pub status: String,
}

impl RlmProgressEvent {
    /// Fraction of the iteration budget consumed (0.0 – 1.0).
    ///
    /// Returns `0.0` when `max_iterations == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::RlmProgressEvent;
    /// use uuid::Uuid;
    ///
    /// let p = RlmProgressEvent {
    ///     trace_id: Uuid::nil(),
    ///     iteration: 5,
    ///     max_iterations: 10,
    ///     status: "x".into(),
    /// };
    /// assert!((p.fraction() - 0.5).abs() < 1e-9);
    /// ```
    pub fn fraction(&self) -> f64 {
        if self.max_iterations == 0 {
            0.0
        } else {
            (self.iteration as f64 / self.max_iterations as f64).clamp(0.0, 1.0)
        }
    }
}

/// Why a Recursive Language Model loop finished.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::RlmOutcome;
///
/// assert!(RlmOutcome::Converged.is_success());
/// assert!(!RlmOutcome::Exhausted.is_success());
/// assert!(!RlmOutcome::Aborted.is_success());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RlmOutcome {
    /// The model produced a final answer within the iteration budget.
    Converged,
    /// The loop hit `max_iterations` without converging. Best-effort
    /// partial summary is still returned.
    Exhausted,
    /// A sub-LLM or tool call failed irrecoverably.
    Failed,
    /// The caller signalled cancellation via the abort watch.
    Aborted,
}

impl RlmOutcome {
    /// Returns `true` only for [`Self::Converged`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::RlmOutcome;
    ///
    /// assert!(RlmOutcome::Converged.is_success());
    /// assert!(!RlmOutcome::Failed.is_success());
    /// ```
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Converged)
    }
}

/// Terminal record for a single RLM invocation.
///
/// Persisted durably so cost post-mortems and trace-driven tuning jobs
/// can reconstruct how each compaction or big-tool-output run behaved.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::{RlmCompletion, RlmOutcome};
/// use uuid::Uuid;
///
/// let c = RlmCompletion {
///     trace_id: Uuid::nil(),
///     outcome: RlmOutcome::Converged,
///     iterations: 4,
///     subcalls: 1,
///     input_tokens: 42_000,
///     output_tokens: 2_100,
///     elapsed_ms: 3_412,
///     reason: None,
///     root_model: "zai/glm-5".into(),
///     subcall_model_used: None,
/// };
/// assert!(c.compression_ratio() > 0.0);
/// assert!(c.compression_ratio() < 1.0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmCompletion {
    /// Correlates this completion with its progress events.
    pub trace_id: Uuid,
    /// Why the loop finished.
    pub outcome: RlmOutcome,
    /// Iterations actually executed.
    pub iterations: usize,
    /// Sub-LLM calls issued during the loop.
    pub subcalls: usize,
    /// Input tokens (the raw blob handed to the router).
    pub input_tokens: usize,
    /// Output tokens (the produced summary / final answer).
    pub output_tokens: usize,
    /// Total wall-clock duration in milliseconds.
    pub elapsed_ms: u64,
    /// Human-readable explanation for non-[`RlmOutcome::Converged`] outcomes.
    pub reason: Option<String>,
    /// Model used for iteration 1 (the root model).
    pub root_model: String,
    /// Model used for iterations ≥ 2, if
    /// [`RlmConfig::subcall_model`](crate::rlm::RlmConfig::subcall_model)
    /// was configured and resolved. `None` means all iterations used
    /// [`Self::root_model`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcall_model_used: Option<String>,
}

impl RlmCompletion {
    /// `output_tokens / input_tokens`, clamped so zero-input loops return
    /// `0.0` rather than `NaN`.
    ///
    /// Values below `1.0` indicate compression; above `1.0` indicate
    /// expansion (rare, but possible when the model rewrites a terse blob
    /// into a verbose explanation).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::{RlmCompletion, RlmOutcome};
    /// use uuid::Uuid;
    ///
    /// let c = RlmCompletion {
    ///     trace_id: Uuid::nil(),
    ///     outcome: RlmOutcome::Converged,
    ///     iterations: 1, subcalls: 0,
    ///     input_tokens: 1000, output_tokens: 100,
    ///     elapsed_ms: 0, reason: None,
    ///     root_model: "m".into(), subcall_model_used: None,
    /// };
    /// assert!((c.compression_ratio() - 0.1).abs() < 1e-9);
    /// ```
    pub fn compression_ratio(&self) -> f64 {
        if self.input_tokens == 0 {
            0.0
        } else {
            self.output_tokens as f64 / self.input_tokens as f64
        }
    }
}

/// Emitted when a configured `subcall_model` could not be resolved,
/// forcing the router to fall back to the root model.
///
/// Durable — a misconfigured subcall model is a cost problem (root models
/// are typically more expensive) that warrants alerting.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::RlmSubcallFallback;
/// use uuid::Uuid;
///
/// let fb = RlmSubcallFallback {
///     trace_id: Uuid::nil(),
///     configured: "minimax-fast".into(),
///     using: "zai/glm-5".into(),
///     reason: "provider not found in registry".into(),
/// };
/// assert_ne!(fb.configured, fb.using);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmSubcallFallback {
    /// Correlates with the enclosing RLM run.
    pub trace_id: Uuid,
    /// The `subcall_model` string from config that failed to resolve.
    pub configured: String,
    /// The root model actually used instead.
    pub using: String,
    /// Why resolution failed (e.g. "provider not found", "rate limited").
    pub reason: String,
}

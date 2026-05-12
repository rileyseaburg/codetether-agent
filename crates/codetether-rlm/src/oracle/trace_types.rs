//! Validated trace and oracle result enums.
//!
//! [`ValidatedTrace`] carries every datum needed to reproduce
//! and audit an RLM REPL session.  [`OracleResult`] wraps the
//! trace with a verdict so the storage layer can partition
//! golden vs. failed records.
//!
//! # Examples
//!
//! ```ignore
//! match &result {
//!     OracleResult::Golden(t) => save_golden(t),
//!     OracleResult::Failed { reason, .. } => log_failure(reason),
//!     _ => {}
//! }
//! ```

use serde::{Deserialize, Serialize};

use super::schema::FinalPayload;
use super::types::{TraceStep, VerificationMethod};

/// Complete snapshot of a validated RLM REPL session.
///
/// Contains the original prompt, every tool step, token
/// accounting, timing, and the oracle's verdict.  Serialised
/// as `.jsonl` for the golden-trace spool.
///
/// # Examples
///
/// ```ignore
/// let trace = ValidatedTrace {
///     prompt: "find async fns".into(),
///     verdict: "golden".into(),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedTrace {
    /// The user query that initiated the REPL session.
    pub prompt: String,
    /// Ordered tool invocations recorded during the session.
    pub trace: Vec<TraceStep>,
    /// Parsed FINAL() payload, if one was emitted.
    pub final_payload: Option<FinalPayload>,
    /// Oracle classification: `"golden"`, `"failed"`, etc.
    pub verdict: String,
    /// Unified diff when the oracle disagrees with the answer.
    pub oracle_diff: Option<String>,
    /// Git SHA of the repository at validation time.
    pub repo_revision: String,
    /// ISO-8601 timestamp of the validation run.
    pub timestamp: String,
    /// Raw FINAL() answer text.
    pub answer: String,
    /// Number of REPL loop iterations executed.
    pub iterations: usize,
    /// Count of LLM sub-queries spawned.
    pub subcalls: usize,
    /// Total input tokens consumed across all LLM calls.
    pub input_tokens: u64,
    /// Total output tokens generated across all LLM calls.
    pub output_tokens: u64,
    /// Wall-clock milliseconds for the full session.
    pub elapsed_ms: u64,
    /// Filesystem path of the primary source file, if any.
    pub source_path: Option<String>,
    /// Which oracle backend produced the verdict.
    pub verification_method: VerificationMethod,
    /// Unique identifier for this trace.
    pub trace_id: String,
}

/// Outcome of oracle validation for a single REPL session.
///
/// Each variant carries the full [`ValidatedTrace`] so
/// downstream consumers always have the raw data regardless
/// of the verdict.
///
/// # Examples
///
/// ```ignore
/// match result {
///     OracleResult::Golden(t) => export_golden(&t),
///     OracleResult::Failed { reason, .. } => warn!("{reason}"),
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone)]
pub enum OracleResult {
    /// Deterministically verified — safe for training data.
    Golden(ValidatedTrace),
    /// Verified via multi-run agreement with a confidence ratio.
    Consensus {
        /// The validated trace snapshot.
        trace: ValidatedTrace,
        /// Fraction of runs that agreed (0.0–1.0).
        agreement_ratio: f32,
    },
    /// No oracle could verify; kept for manual review.
    Unverified {
        /// Why verification was skipped.
        reason: String,
        /// The unverified trace snapshot.
        trace: ValidatedTrace,
    },
    /// Oracle actively disagrees with the answer.
    Failed {
        /// What the oracle found wrong.
        reason: String,
        /// Optional unified diff of expected vs. actual.
        diff: Option<String>,
        /// The failing trace snapshot.
        trace: ValidatedTrace,
    },
}

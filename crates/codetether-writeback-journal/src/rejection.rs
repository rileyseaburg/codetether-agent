//! Typed reasons to reject a staged writeback.
use serde::{Deserialize, Serialize};

/// Reason codes for a rejected writeback.
/// # Examples
/// ```rust
/// use codetether_writeback_journal::RejectReason;
/// let reason = RejectReason::DestructiveOp;
/// assert!(matches!(reason, RejectReason::DestructiveOp));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum RejectReason {
    /// A destructive overwrite was requested without an explicit
    /// `allow_destructive` flag.
    DestructiveOp,
    /// Schema validation failed.
    SchemaMismatch { detail: String },
    /// Provenance / authorship check failed.
    ProvenanceFail { detail: String },
    /// The op would drop a page below its `min_fidelity` invariant.
    InvariantViolation { detail: String },
}

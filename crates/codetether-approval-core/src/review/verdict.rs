//! Structured outcome of an approval review.

use serde::{Deserialize, Serialize};

/// What the reviewer recommends.
///
/// Variants indicate approval, requested revisions, or inability to decide.
/// # Examples
/// ```
/// use codetether_approval_core::review::ReviewOutcome;
/// let message = match ReviewOutcome::Approve {
///     ReviewOutcome::Approve => "approved",
///     ReviewOutcome::RequestChanges => "revise",
///     ReviewOutcome::Escalate => "needs attention",
/// };
/// assert_eq!(message, "approved");
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewOutcome {
    /// The change serves the goal and is safe to apply.
    Approve,
    /// The change should be revised; `reason` tells the author how.
    RequestChanges,
    /// The reviewer could not decide (unclear, out of scope, timed out).
    Escalate,
}

impl ReviewOutcome {
    /// Human-readable label for the recommendation.
    pub fn label(self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::RequestChanges => "request changes",
            Self::Escalate => "escalate",
        }
    }
}

/// A complete reviewer verdict.
///
/// # Examples
/// ```
/// use codetether_approval_core::review::{ReviewVerdict, ReviewOutcome};
/// let verdict = ReviewVerdict::escalate("missing context");
/// assert_eq!(verdict.outcome, ReviewOutcome::Escalate);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReviewVerdict {
    /// Advisory recommendation; this does not grant tool authorization.
    pub outcome: ReviewOutcome,
    /// One or two sentences a human can act on.
    pub reason: String,
    /// Specific observations, each standing alone.
    #[serde(default)]
    pub findings: Vec<String>,
}

impl ReviewVerdict {
    /// The verdict used when the reviewer cannot finish on its own.
    pub fn escalate(reason: impl Into<String>) -> Self {
        Self {
            outcome: ReviewOutcome::Escalate,
            reason: reason.into(),
            findings: Vec::new(),
        }
    }
}

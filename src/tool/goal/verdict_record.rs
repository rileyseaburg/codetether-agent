//! One persisted verifier decision on a goal transition.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Durable record of a single verification, stored in the verdict log.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verdict_log::VerdictRecord;
///
/// let r = VerdictRecord::new("g1", "complete", false, "bedrock/m", "VERDICT: FAIL");
/// assert!(!r.passed && !r.escalated);
/// assert_eq!(r.report_sha256.len(), 64);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerdictRecord {
    /// When the verdict was recorded.
    pub at: DateTime<Utc>,
    /// Goal the verdict applies to.
    pub goal_id: String,
    /// Claimed transition (`complete` or `blocked`).
    pub claimed: String,
    /// Whether the verifier returned PASS.
    pub passed: bool,
    /// Verifier identity, usually its model.
    pub verifier: String,
    /// SHA-256 of the full verifier report.
    pub report_sha256: String,
    /// Marks the attempt cap being hit; resets the rejection count.
    #[serde(default)]
    pub escalated: bool,
    /// Infrastructure/protocol failure, excluded from the rejection budget.
    #[serde(default)]
    pub unavailable: bool,
}

impl VerdictRecord {
    /// Build a record for a completed verification.
    pub fn new(goal: &str, claimed: &str, passed: bool, verifier: &str, report: &str) -> Self {
        Self {
            at: Utc::now(),
            goal_id: goal.into(),
            claimed: claimed.into(),
            passed,
            verifier: verifier.into(),
            report_sha256: hex::encode(Sha256::digest(report.as_bytes())),
            escalated: false,
            unavailable: false,
        }
    }
}

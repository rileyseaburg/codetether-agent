//! Choose recovery behavior for claims requiring passing proof.
use super::claim::EvidenceClaim;
use super::level::EvidenceLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Next action when a requested proof has not been established.
/// # Examples
/// ```
/// use codetether_evidence_core::{claim::assess_claim, recovery::{recovery_for, RecoveryAction}};
/// assert_eq!(recovery_for(&assess_claim("Argo"), true), RecoveryAction::ContinueLiveRun);
/// ```
pub enum RecoveryAction {
    /// Continue obtaining live deployment or platform evidence.
    ContinueLiveRun,
    /// Report the blocker for the current evidence category.
    ReportBlocker,
    /// Passing proof was not requested.
    NoRecoveryNeeded,
}

/// Select recovery for a claim and the user's passing-proof requirement.
/// # Arguments
/// `claim` supplies the category; `requested_passing_proof` enables recovery.
/// # Returns
/// The established live-run, blocker-reporting or no-recovery action.
/// # Examples
/// ```
/// use codetether_evidence_core::{claim::assess_claim, recovery::{recovery_for, RecoveryAction}};
/// assert_eq!(recovery_for(&assess_claim("local"), false), RecoveryAction::NoRecoveryNeeded);
/// ```
pub fn recovery_for(claim: &EvidenceClaim, requested_passing_proof: bool) -> RecoveryAction {
    if !requested_passing_proof {
        return RecoveryAction::NoRecoveryNeeded;
    }
    if matches!(
        claim.level,
        EvidenceLevel::LiveDeploymentArgo | EvidenceLevel::RealPlatformUpload
    ) {
        RecoveryAction::ContinueLiveRun
    } else {
        RecoveryAction::ReportBlocker
    }
}

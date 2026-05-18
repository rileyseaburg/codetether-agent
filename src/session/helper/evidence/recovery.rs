use super::claim::EvidenceClaim;
use super::level::EvidenceLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryAction {
    ContinueLiveRun,
    ReportBlocker,
    NoRecoveryNeeded,
}

pub(crate) fn recovery_for(claim: &EvidenceClaim, requested_passing_proof: bool) -> RecoveryAction {
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

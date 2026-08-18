//! Stable wire labels for [`ThoughtEventType`](super::ThoughtEventType).

use super::ThoughtEventType;

impl ThoughtEventType {
    /// Return the stable snake_case label used in logs and payloads.
    pub(in crate::cognition) fn as_str(&self) -> &'static str {
        match self {
            Self::ThoughtGenerated => "thought_generated",
            Self::HypothesisRaised => "hypothesis_raised",
            Self::CheckRequested => "check_requested",
            Self::CheckResult => "check_result",
            Self::ProposalCreated => "proposal_created",
            Self::ProposalVerified => "proposal_verified",
            Self::ProposalRejected => "proposal_rejected",
            Self::ActionExecuted => "action_executed",
            Self::PersonaSpawned => "persona_spawned",
            Self::PersonaReaped => "persona_reaped",
            Self::SnapshotCompressed => "snapshot_compressed",
            Self::BeliefExtracted => "belief_extracted",
            Self::BeliefContested => "belief_contested",
            Self::BeliefRevalidated => "belief_revalidated",
            Self::BudgetPaused => "budget_paused",
            Self::IdleReaped => "idle_reaped",
            Self::AttentionCreated => "attention_created",
            Self::VoteCast => "vote_cast",
            Self::WorkspaceUpdated => "workspace_updated",
        }
    }
}

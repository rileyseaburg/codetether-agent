//! Governance fixtures for proposal resolution tests.

use chrono::{Duration as ChronoDuration, Utc};
use std::collections::HashMap;
use std::time::Duration;

use super::{
    CognitionRuntime, Proposal, ProposalRisk, ProposalStatus, ProposalVote, SwarmGovernance,
};

/// Governance where auditors hold a veto and quorum is half the swarm.
pub(super) fn auditor_veto_governance() -> SwarmGovernance {
    SwarmGovernance {
        quorum_fraction: 0.5,
        required_approvers_by_role: HashMap::new(),
        veto_roles: vec!["auditor".to_string()],
        vote_timeout_secs: 300,
    }
}

/// A `Created` proposal carrying `votes` and a live deadline.
pub(super) fn pending_proposal(
    id: &str,
    author: &str,
    votes: HashMap<String, ProposalVote>,
) -> Proposal {
    Proposal {
        id: id.to_string(),
        persona_id: author.to_string(),
        title: format!("{id} title"),
        rationale: "testing governance".to_string(),
        evidence_refs: Vec::new(),
        risk: ProposalRisk::Low,
        status: ProposalStatus::Created,
        created_at: Utc::now(),
        votes,
        vote_deadline: Some(Utc::now() + ChronoDuration::seconds(300)),
        votes_requested: true,
        quorum_needed: 1,
    }
}

/// Run the loop long enough for one governance sweep.
pub(super) async fn tick_once(runtime: &CognitionRuntime) {
    runtime.start(None).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    runtime.stop(None).await.unwrap();
}

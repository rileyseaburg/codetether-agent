//! Proposal creation from a Reflect-phase thought.

use chrono::{Duration as ChronoDuration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    Proposal, ProposalRisk, ProposalStatus, SwarmGovernance, ThoughtResult, ThoughtWorkItem,
    defaults, text_util,
};

/// Build a Low-risk proposal from `thought`, freezing quorum at creation time.
pub(super) fn build_proposal(
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    governance: &SwarmGovernance,
    active_persona_count: usize,
) -> Proposal {
    Proposal {
        id: Uuid::new_v4().to_string(),
        persona_id: work.persona_id.clone(),
        title: defaults::proposal_title_from_thought(&thought.thinking, work.thought_count),
        rationale: text_util::trim_for_storage(&thought.thinking, 900),
        evidence_refs: vec!["internal.thought_stream".to_string()],
        risk: ProposalRisk::Low,
        status: ProposalStatus::Created,
        created_at: Utc::now(),
        votes: HashMap::new(),
        vote_deadline: Some(
            Utc::now() + ChronoDuration::seconds(governance.vote_timeout_secs as i64),
        ),
        votes_requested: false,
        quorum_needed: (active_persona_count as f32 * governance.quorum_fraction).ceil() as usize,
    }
}

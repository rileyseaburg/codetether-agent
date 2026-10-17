//! Proposal creation from a Reflect-phase thought.

use crate::proposal::title as defaults;
use crate::text as text_util;
use chrono::{Duration as ChronoDuration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    Proposal, ProposalRisk, ProposalStatus, SwarmGovernance, ThoughtResult, ThoughtWorkItem,
};

/// Build a Low-risk proposal from `thought`, freezing quorum at creation time.
///
/// # Arguments
/// * `work` - Persona, phase and lineage associated with the completed work.
/// * `thought` - Completed thought and its provenance.
/// * `governance` - Voting policy used to freeze quorum and deadline.
/// * `active_persona_count` - Eligible persona count used when computing quorum.
///
/// # Returns
/// The constructed record with the existing field and provenance rules.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::build::build_proposal;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Reflect };
/// let thought = ThoughtResult { source: "example", model: None, finish_reason: None, thinking: "Review the change".into(), prompt_tokens: None, completion_tokens: None, total_tokens: None, latency_ms: 0, error: None };
/// let proposal = build_proposal(&work, &thought, &SwarmGovernance::default(), 4);
/// assert_eq!(proposal.quorum_needed, 2);
/// ```
pub fn build_proposal(
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

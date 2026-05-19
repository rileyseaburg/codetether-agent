//! Phase C step 31 — order the persona relay chain by LCB score.
//!
//! Static [`default_chain`] returns architect → implementer → reviewer.
//! This module replaces "always run all three in fixed order" with
//! "score each persona under the current bucket, run the highest-LCB
//! candidate first, fall back to the static chain when delegation is
//! disabled or no posterior has any data". The relay is short, so the
//! LCB only governs *order*, not membership.
//!
//! [`default_chain`]: super::persona::default_chain

use crate::session::delegation::DelegationState;
use crate::session::delegation_skills::AUTOCHAT_PERSONA;
use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

use super::persona::{Persona, default_chain};

#[cfg(test)]
#[path = "persona_pick_tests.rs"]
mod tests;

/// Synthetic relay bucket — autochat personas are not bucket-conditioned
/// today, so all calls share one medium-difficulty / no-tool projection.
pub fn relay_bucket() -> Bucket {
    Bucket {
        difficulty: Difficulty::Medium,
        dependency: Dependency::Isolated,
        tool_use: ToolUse::No,
    }
}

/// LCB-rank the relay chain in place. Returns the chain unchanged when
/// delegation is disabled or all personas tie at the cold-start prior.
pub fn rank_chain(state: &DelegationState, bucket: Bucket) -> Vec<Persona> {
    let chain: Vec<Persona> = default_chain().to_vec();
    if !state.enabled() {
        return chain;
    }
    let mut scored: Vec<(Persona, f64)> = chain
        .into_iter()
        .map(|p| {
            let score = state.score(p.name, AUTOCHAT_PERSONA, bucket).unwrap_or(0.0);
            (p, score)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.into_iter().map(|(p, _)| p).collect()
}

/// Record the relay outcome for the persona that ran last.
pub fn record_outcome(state: &mut DelegationState, persona: &str, bucket: Bucket, success: bool) {
    if !state.enabled() {
        return;
    }
    state.update(persona, AUTOCHAT_PERSONA, bucket, success);
}

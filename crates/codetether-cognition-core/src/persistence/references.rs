//! Gather the event IDs needed to preserve belief and proposal evidence.
use crate::{Proposal, beliefs::Belief};
use std::collections::HashMap;

/// Collect event IDs referenced by beliefs and proposals.
pub(super) fn referenced_event_ids(
    beliefs: &HashMap<String, Belief>,
    proposals: &HashMap<String, Proposal>,
) -> std::collections::HashSet<String> {
    let mut ids = std::collections::HashSet::new();
    for belief in beliefs.values() {
        for ref_id in &belief.evidence_refs {
            ids.insert(ref_id.clone());
        }
    }
    for proposal in proposals.values() {
        for ref_id in &proposal.evidence_refs {
            ids.insert(ref_id.clone());
        }
    }
    ids
}

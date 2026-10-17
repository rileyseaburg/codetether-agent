//! Preserve referenced evidence first, followed by unseen tail events.
use super::state::TAIL_EVENTS;
use super::trim::trim_event_for_storage;
use crate::ThoughtEvent;
use std::collections::{HashSet, VecDeque};

pub(super) fn select_evidence(
    events_snap: &VecDeque<ThoughtEvent>,
    ref_ids: &HashSet<String>,
) -> Vec<ThoughtEvent> {
    let mut evidence_events: Vec<ThoughtEvent> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Referenced events first
    for event in events_snap.iter() {
        if ref_ids.contains(&event.id) && seen.insert(event.id.clone()) {
            evidence_events.push(trim_event_for_storage(event));
        }
    }

    // Then tail events
    let tail_start = events_snap.len().saturating_sub(TAIL_EVENTS);
    for event in events_snap.iter().skip(tail_start) {
        if seen.insert(event.id.clone()) {
            evidence_events.push(trim_event_for_storage(event));
        }
    }

    evidence_events
}

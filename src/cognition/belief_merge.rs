//! Belief extraction merge during the Reflect phase.

use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use super::belief_confirm::{confirm_existing, existing_id};
use super::belief_contest::apply_contradictions;
use super::beliefs::Belief;
use super::{AttentionItem, ThoughtEvent, ThoughtEventType, ThoughtWorkItem, text_util};

/// Merge extracted beliefs into the store, confirming duplicates and applying
/// contradictions. Returns events for newly created beliefs.
pub(super) fn merge_beliefs(
    store: &mut HashMap<String, Belief>,
    attention: &mut Vec<AttentionItem>,
    work: &ThoughtWorkItem,
    extracted: Vec<Belief>,
) -> Vec<ThoughtEvent> {
    let mut events = Vec::new();
    for mut belief in extracted {
        match existing_id(store, &belief) {
            Some(id) => confirm_existing(store, &id, &work.persona_id),
            None => {
                apply_contradictions(store, attention, &mut belief);
                events.push(extracted_event(work, &belief));
                belief.clamp_confidence();
                store.insert(belief.id.clone(), belief);
            }
        }
    }
    events
}

fn extracted_event(work: &ThoughtWorkItem, belief: &Belief) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::BeliefExtracted,
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp: Utc::now(),
        payload: json!({
            "belief_id": belief.id,
            "belief_key": belief.belief_key,
            "claim": text_util::trim_for_storage(&belief.claim, 280),
            "confidence": belief.confidence,
        }),
    }
}

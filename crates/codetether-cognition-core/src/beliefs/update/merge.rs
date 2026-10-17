//! Belief extraction merge during the Reflect phase.

use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use super::confirm::{confirm_existing, existing_id};
use super::contest::apply_contradictions;
use crate::beliefs::Belief;
use crate::{AttentionItem, ThoughtEvent, ThoughtEventType, ThoughtWorkItem, text as text_util};

/// Merge extracted beliefs into the store, confirming duplicates and applying
/// contradictions. Returns events for newly created beliefs.
///
/// # Arguments
/// * `store` - Beliefs indexed by record ID.
/// * `attention` - Attention queue to extend.
/// * `work` - Persona and phase associated with extracted beliefs.
/// * `extracted` - New beliefs to merge in order.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::beliefs::update::merge_beliefs;
/// let mut store = std::collections::HashMap::new();
/// let mut attention = vec![];
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Example".into(), role: "reviewer".into(), charter: "Review".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Reflect };
/// let result = merge_beliefs(&mut store, &mut attention, &work, vec![]);
/// assert!(result.is_empty());
/// ```
pub fn merge_beliefs(
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

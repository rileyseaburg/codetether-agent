//! Idle-TTL reaping of personas that stopped making progress.

use chrono::{DateTime, Utc};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use super::idle_detect::{children_of, is_idle, mark_reaped};
use super::{PersonaRuntimeState, ThoughtEvent, ThoughtEventType};

/// Reap personas idle past their TTL, cascading to direct children.
pub(super) fn reap_idle(
    personas: &mut HashMap<String, PersonaRuntimeState>,
    now: DateTime<Utc>,
) -> Vec<ThoughtEvent> {
    let idle_ids: Vec<String> = personas
        .values()
        .filter(|p| is_idle(p, now))
        .map(|p| p.identity.id.clone())
        .collect();

    for id in &idle_ids {
        mark_reaped(personas, id, now);
        for child_id in children_of(personas, id) {
            mark_reaped(personas, &child_id, now);
        }
    }
    idle_ids.into_iter().map(|id| event(id, now)).collect()
}

fn event(persona_id: String, now: DateTime<Utc>) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::IdleReaped,
        persona_id: Some(persona_id),
        swarm_id: None,
        timestamp: now,
        payload: json!({ "reason": "idle_ttl_expired" }),
    }
}

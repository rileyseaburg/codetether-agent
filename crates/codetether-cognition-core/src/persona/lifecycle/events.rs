//! Lifecycle events constructed immediately before application publication.
use crate::{PersonaRuntimeState, ThoughtEvent, ThoughtEventType};
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

///
/// # Arguments
/// * `persona` - New persona whose creation is announced.
/// * `now` - Timestamp shared by the operation.
///
/// # Returns
/// The constructed lifecycle record or event.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::persona::lineage::Inherited;
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example(persona: &PersonaRuntimeState, now: chrono::DateTime<Utc>) {
/// let _result = codetether_cognition_core::persona::lifecycle::spawned_event(persona, now);
/// # }
/// ```
pub fn spawned_event(persona: &PersonaRuntimeState, now: chrono::DateTime<Utc>) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::PersonaSpawned,
        persona_id: Some(persona.identity.id.clone()),
        swarm_id: persona.identity.swarm_id.clone(),
        timestamp: now,
        payload: json!({
            "name": persona.identity.name,
            "role": persona.identity.role,
            "depth": persona.identity.depth,
        }),
    }
}

/// Construct a reap event for one selected persona.
///
/// # Arguments
/// * `id` - Reaped persona identifier.
/// * `reason` - Reason recorded in the event.
/// * `cascade` - Whether descendant reaping was requested.
/// * `now` - Timestamp shared by the operation.
///
/// # Returns
/// The constructed lifecycle record or event.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::persona::lineage::Inherited;
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example(id: &str, reason: &str, cascade: bool, now: DateTime<Utc>) {
/// let _result = codetether_cognition_core::persona::lifecycle::reaped_event(id, reason, cascade, now);
/// # }
/// ```
pub fn reaped_event(id: &str, reason: &str, cascade: bool, now: DateTime<Utc>) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::PersonaReaped,
        persona_id: Some(id.to_string()),
        swarm_id: None,
        timestamp: now,
        payload: json!({ "reason": reason, "cascade": cascade }),
    }
}

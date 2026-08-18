//! Idle-persona detection and reap marking.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::{PersonaRuntimeState, PersonaStatus};

/// Whether a persona has stalled past its idle TTL.
///
/// Budget-paused personas are exempt: they are throttled, not stalled.
pub(super) fn is_idle(persona: &PersonaRuntimeState, now: DateTime<Utc>) -> bool {
    persona.status == PersonaStatus::Active
        && !persona.budget_paused
        && now
            .signed_duration_since(persona.last_progress_at)
            .num_seconds()
            > persona.policy.idle_ttl_secs as i64
}

/// Mark one persona reaped.
pub(super) fn mark_reaped(
    personas: &mut HashMap<String, PersonaRuntimeState>,
    id: &str,
    now: DateTime<Utc>,
) {
    if let Some(persona) = personas.get_mut(id) {
        persona.status = PersonaStatus::Reaped;
        persona.updated_at = now;
    }
}

/// Direct children of `id`.
pub(super) fn children_of(
    personas: &HashMap<String, PersonaRuntimeState>,
    id: &str,
) -> Vec<String> {
    personas
        .values()
        .filter(|p| p.identity.parent_id.as_deref() == Some(id))
        .map(|p| p.identity.id.clone())
        .collect()
}

//! Idle-persona detection and reap marking.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::{PersonaRuntimeState, PersonaStatus};

/// Whether a persona has stalled past its idle TTL.
///
/// Budget-paused personas are exempt: they are throttled, not stalled.
///
/// # Arguments
/// * `persona` - Persona state to inspect or update.
/// * `now` - Time used for this operation.
///
/// # Returns
/// Whether this active, unpaused persona exceeded its idle limit.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(persona: &PersonaRuntimeState, now: DateTime<Utc>) {
/// let _result = codetether_cognition_core::persona::scheduling::is_idle(persona, now);
/// # }
/// ```
pub fn is_idle(persona: &PersonaRuntimeState, now: DateTime<Utc>) -> bool {
    persona.status == PersonaStatus::Active
        && !persona.budget_paused
        && now
            .signed_duration_since(persona.last_progress_at)
            .num_seconds()
            > persona.policy.idle_ttl_secs as i64
}

/// Mark one persona reaped.
///
/// # Arguments
/// * `personas` - Persona records guarded by the caller or supplied shared lock.
/// * `id` - Persona identifier.
/// * `now` - Time used for this operation.
///
/// # Returns
/// Updates the supplied state; no separate value is returned.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(personas: &mut HashMap<String, PersonaRuntimeState>, id: &str, now: DateTime<Utc>,) {
/// let _result = codetether_cognition_core::persona::scheduling::mark_reaped(personas, id, now);
/// # }
/// ```
pub fn mark_reaped(
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
///
/// # Arguments
/// * `personas` - Persona records guarded by the caller or supplied shared lock.
/// * `id` - Persona identifier.
///
/// # Returns
/// IDs of direct children in the existing store order.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(personas: &HashMap<String, PersonaRuntimeState>, id: &str) {
/// let _result = codetether_cognition_core::persona::scheduling::children_of(personas, id);
/// # }
/// ```
pub fn children_of(personas: &HashMap<String, PersonaRuntimeState>, id: &str) -> Vec<String> {
    personas
        .values()
        .filter(|p| p.identity.parent_id.as_deref() == Some(id))
        .map(|p| p.identity.id.clone())
        .collect()
}

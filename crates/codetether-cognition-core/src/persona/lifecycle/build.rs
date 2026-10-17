//! Persona state assembly from a validated create request.

use chrono::{DateTime, Utc};

use crate::persona::lineage::Inherited;
use crate::{
    CreatePersonaRequest, PersonaIdentity, PersonaPolicy, PersonaRuntimeState, PersonaStatus,
};

/// Build a fresh active persona from a validated request.
///
/// # Arguments
/// * `persona_id` - Requested persona identifier.
/// * `req` - Validated or pending lifecycle request.
/// * `inherited` - Parent lineage and inherited resource policy.
/// * `policy` - Effective persona resource policy.
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
/// # async fn example( persona_id: String, req: CreatePersonaRequest, inherited: Inherited, policy: PersonaPolicy, now: DateTime<Utc>, ) {
/// let _result = codetether_cognition_core::persona::lifecycle::build_persona(persona_id, req, inherited, policy, now);
/// # }
/// ```
pub fn build_persona(
    persona_id: String,
    req: CreatePersonaRequest,
    inherited: Inherited,
    policy: PersonaPolicy,
    now: DateTime<Utc>,
) -> PersonaRuntimeState {
    PersonaRuntimeState {
        identity: PersonaIdentity {
            id: persona_id,
            name: req.name,
            role: req.role,
            charter: req.charter,
            swarm_id: req.swarm_id.or(inherited.swarm_id),
            parent_id: req.parent_id,
            depth: inherited.depth,
            created_at: now,
            tags: req.tags,
        },
        policy,
        status: PersonaStatus::Active,
        thought_count: 0,
        last_tick_at: None,
        updated_at: now,
        tokens_this_window: 0,
        compute_ms_this_window: 0,
        window_started_at: now,
        last_progress_at: now,
        budget_paused: false,
    }
}

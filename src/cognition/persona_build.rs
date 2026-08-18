//! Persona state assembly from a validated create request.

use chrono::{DateTime, Utc};

use super::persona_lineage::Inherited;
use super::{
    CreatePersonaRequest, PersonaIdentity, PersonaPolicy, PersonaRuntimeState, PersonaStatus,
};

/// Build a fresh active persona from a validated request.
pub(super) fn build_persona(
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

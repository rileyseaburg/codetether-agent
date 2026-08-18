//! Persona fixtures for budget window tests.

use chrono::{Duration as ChronoDuration, Utc};
use std::collections::HashMap;

use super::{PersonaIdentity, PersonaPolicy, PersonaRuntimeState, PersonaStatus};

/// A persona whose budget window opened `window_age_secs` ago.
pub(super) fn persona(window_age_secs: i64, tokens: u32, compute_ms: u32) -> PersonaRuntimeState {
    let now = Utc::now();
    PersonaRuntimeState {
        identity: PersonaIdentity {
            id: "p1".to_string(),
            name: "test".to_string(),
            role: "tester".to_string(),
            charter: "test".to_string(),
            swarm_id: None,
            parent_id: None,
            depth: 0,
            created_at: now,
            tags: Vec::new(),
        },
        policy: PersonaPolicy::default(),
        status: PersonaStatus::Active,
        thought_count: 0,
        last_tick_at: None,
        updated_at: now,
        tokens_this_window: tokens,
        compute_ms_this_window: compute_ms,
        window_started_at: now - ChronoDuration::seconds(window_age_secs),
        last_progress_at: now,
        budget_paused: false,
    }
}

/// Wrap one persona in a keyed store.
pub(super) fn store(persona: PersonaRuntimeState) -> HashMap<String, PersonaRuntimeState> {
    HashMap::from([("p1".to_string(), persona)])
}

//! Budget window accounting for one persona.

use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

use super::{PersonaRuntimeState, ThoughtEvent, ThoughtEventType};

/// Budget windows reset on this cadence.
const WINDOW_SECS: i64 = 60;

/// Clear the token/compute counters once the window has elapsed.
pub(super) fn reset_window_if_elapsed(persona: &mut PersonaRuntimeState, now: DateTime<Utc>) {
    let elapsed = now
        .signed_duration_since(persona.window_started_at)
        .num_seconds();
    if elapsed >= WINDOW_SECS {
        persona.tokens_this_window = 0;
        persona.compute_ms_this_window = 0;
        persona.window_started_at = now;
    }
}

/// Whether the persona has exhausted its token or compute budget.
pub(super) fn over_budget(persona: &PersonaRuntimeState) -> bool {
    persona.tokens_this_window >= persona.policy.token_budget_per_minute
        || persona.compute_ms_this_window >= persona.policy.compute_ms_per_minute
}

/// Event announcing that a persona was paused for exceeding its budget.
pub(super) fn budget_paused_event(
    persona: &PersonaRuntimeState,
    now: DateTime<Utc>,
) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::BudgetPaused,
        persona_id: Some(persona.identity.id.clone()),
        swarm_id: persona.identity.swarm_id.clone(),
        timestamp: now,
        payload: json!({
            "budget_paused": true,
            "tokens_used": persona.tokens_this_window,
            "compute_ms_used": persona.compute_ms_this_window,
            "token_budget": persona.policy.token_budget_per_minute,
            "compute_budget": persona.policy.compute_ms_per_minute,
        }),
    }
}

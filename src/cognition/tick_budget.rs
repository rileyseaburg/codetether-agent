//! Per-tick work selection across active personas.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::tick_window::{budget_paused_event, over_budget, reset_window_if_elapsed};
use super::{PersonaRuntimeState, PersonaStatus, ThoughtEvent, ThoughtPhase, ThoughtWorkItem};

/// Advance every active persona one tick, skipping those over budget.
///
/// Returns the work items to think about; `events` receives any budget-pause
/// notifications raised during selection.
pub(super) fn select_work(
    personas: &mut HashMap<String, PersonaRuntimeState>,
    now: DateTime<Utc>,
    events: &mut Vec<ThoughtEvent>,
) -> Vec<ThoughtWorkItem> {
    let mut items = Vec::new();
    for persona in personas.values_mut() {
        if persona.status != PersonaStatus::Active {
            continue;
        }
        reset_window_if_elapsed(persona, now);
        if over_budget(persona) {
            if !persona.budget_paused {
                persona.budget_paused = true;
                events.push(budget_paused_event(persona, now));
            }
            continue;
        }
        persona.budget_paused = false;
        persona.thought_count = persona.thought_count.saturating_add(1);
        persona.last_tick_at = Some(now);
        persona.updated_at = now;
        items.push(work_item(persona));
    }
    items
}

fn work_item(persona: &PersonaRuntimeState) -> ThoughtWorkItem {
    ThoughtWorkItem {
        persona_id: persona.identity.id.clone(),
        persona_name: persona.identity.name.clone(),
        role: persona.identity.role.clone(),
        charter: persona.identity.charter.clone(),
        swarm_id: persona.identity.swarm_id.clone(),
        thought_count: persona.thought_count,
        phase: ThoughtPhase::from_thought_count(persona.thought_count),
    }
}

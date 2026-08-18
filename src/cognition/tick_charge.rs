//! Post-thought budget counter accounting.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{PersonaRuntimeState, ThoughtResult};

/// Charge a completed thought's tokens and latency against the persona budget.
///
/// Progress is only recorded for model-backed thoughts, so a persona stuck on
/// deterministic text still ages toward idle reaping.
pub(super) async fn charge_thought(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    persona_id: &str,
    thought: &ThoughtResult,
    model_backed: bool,
) {
    let tokens = thought.total_tokens.unwrap_or(0);
    let compute_ms = thought.latency_ms as u32;
    let mut map = personas.write().await;
    if let Some(persona) = map.get_mut(persona_id) {
        persona.tokens_this_window = persona.tokens_this_window.saturating_add(tokens);
        persona.compute_ms_this_window = persona.compute_ms_this_window.saturating_add(compute_ms);
        if model_backed {
            persona.last_progress_at = Utc::now();
        }
    }
}

//! Marking persona progress so active work delays idle reaping.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::PersonaRuntimeState;

/// Record that `persona_id` made meaningful progress just now.
pub(super) async fn mark_progress(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    persona_id: &str,
) {
    let mut map = personas.write().await;
    if let Some(persona) = map.get_mut(persona_id) {
        persona.last_progress_at = Utc::now();
    }
}

/// Read a persona's allowed tool list, empty when unknown.
pub(super) async fn allowed_tools(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    persona_id: &str,
) -> Vec<String> {
    personas
        .read()
        .await
        .get(persona_id)
        .map(|p| p.policy.allowed_tools.clone())
        .unwrap_or_default()
}

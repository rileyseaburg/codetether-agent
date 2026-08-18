//! Persona reaping, optionally cascading to descendants.

use anyhow::{Result, anyhow};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use super::persona_reap_targets::collect_reap_targets;
use super::{
    CognitionRuntime, PersonaStatus, ReapPersonaRequest, ReapPersonaResponse, ThoughtEvent,
    ThoughtEventType,
};

impl CognitionRuntime {
    /// Reap one persona or its full descendant tree.
    ///
    /// # Errors
    ///
    /// Returns an error when `persona_id` is unknown.
    pub async fn reap_persona(
        &self,
        persona_id: &str,
        req: ReapPersonaRequest,
    ) -> Result<ReapPersonaResponse> {
        let cascade = req.cascade.unwrap_or(false);
        let now = Utc::now();

        let mut personas = self.personas.write().await;
        if !personas.contains_key(persona_id) {
            return Err(anyhow!("Persona not found: {persona_id}"));
        }
        let reaped_ids = collect_reap_targets(&personas, persona_id, cascade);
        for id in &reaped_ids {
            if let Some(persona) = personas.get_mut(id) {
                persona.status = PersonaStatus::Reaped;
                persona.updated_at = now;
            }
        }
        drop(personas);

        let reason = req
            .reason
            .clone()
            .unwrap_or_else(|| "manual_reap".to_string());
        for id in &reaped_ids {
            self.push_event(ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::PersonaReaped,
                persona_id: Some(id.clone()),
                swarm_id: None,
                timestamp: now,
                payload: json!({ "reason": reason, "cascade": cascade }),
            })
            .await;
        }

        Ok(ReapPersonaResponse {
            count: reaped_ids.len(),
            reaped_ids,
        })
    }
}

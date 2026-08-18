//! Persona creation with lineage and policy enforcement.

use anyhow::{Result, anyhow};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use super::persona_build::build_persona;
use super::persona_depth::check_depth;
use super::persona_lineage::{Inherited, inherit_from_parent};
use super::{
    CognitionRuntime, CreatePersonaRequest, PersonaRuntimeState, ThoughtEvent, ThoughtEventType,
};

impl CognitionRuntime {
    /// Create a persona record, inheriting lineage from its parent if any.
    ///
    /// # Errors
    ///
    /// Returns an error when the parent is invalid, a branching or depth limit
    /// is exceeded, or the requested persona ID already exists.
    pub async fn create_persona(&self, req: CreatePersonaRequest) -> Result<PersonaRuntimeState> {
        let now = Utc::now();
        let mut personas = self.personas.write().await;

        let inherited = match req.parent_id.as_deref() {
            Some(parent_id) => inherit_from_parent(&personas, parent_id)?,
            None => Inherited::default(),
        };
        let policy = req
            .policy
            .clone()
            .or_else(|| inherited.policy.clone())
            .unwrap_or_else(|| self.default_policy.clone());
        check_depth(inherited.depth, inherited.policy.as_ref(), &policy)?;

        let persona_id = req
            .persona_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        if personas.contains_key(&persona_id) {
            return Err(anyhow!("Persona id already exists: {persona_id}"));
        }
        let persona = build_persona(persona_id.clone(), req, inherited, policy, now);
        personas.insert(persona_id, persona.clone());
        drop(personas);

        self.push_event(spawned_event(&persona, now)).await;
        Ok(persona)
    }
}

fn spawned_event(persona: &PersonaRuntimeState, now: chrono::DateTime<Utc>) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::PersonaSpawned,
        persona_id: Some(persona.identity.id.clone()),
        swarm_id: persona.identity.swarm_id.clone(),
        timestamp: now,
        payload: json!({
            "name": persona.identity.name,
            "role": persona.identity.role,
            "depth": persona.identity.depth,
        }),
    }
}

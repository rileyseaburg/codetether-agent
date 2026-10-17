//! Validate and insert a persona while holding its store write lock.
use super::build::build_persona;
use crate::persona::{
    depth::check_depth,
    lineage::{Inherited, inherit_from_parent},
};
use crate::{CreatePersonaRequest, PersonaPolicy, PersonaRuntimeState};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Validate and create a persona, releasing the lock before returning.
///
/// # Arguments
/// * `personas` - Shared persona store.
/// * `default_policy` - Default policy when neither request nor parent supplies one.
/// * `req` - Validated or pending lifecycle request.
///
/// # Returns
/// The created record and operation timestamp after releasing the store lock.
///
/// # Errors
/// Rejects invalid parents, exceeded branching/depth limits or duplicate IDs.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::persona::lineage::Inherited;
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example( personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>, default_policy: &PersonaPolicy, req: CreatePersonaRequest, ) {
/// let _result = codetether_cognition_core::persona::lifecycle::create_persona(personas, default_policy, req).await;
/// # }
/// ```
pub async fn create_persona(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    default_policy: &PersonaPolicy,
    req: CreatePersonaRequest,
) -> Result<(PersonaRuntimeState, DateTime<Utc>)> {
    let now = Utc::now();
    let mut personas = personas.write().await;

    let inherited = match req.parent_id.as_deref() {
        Some(parent_id) => inherit_from_parent(&personas, parent_id)?,
        None => Inherited::default(),
    };
    let policy = req
        .policy
        .clone()
        .or_else(|| inherited.policy.clone())
        .unwrap_or_else(|| default_policy.clone());
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

    Ok((persona, now))
}

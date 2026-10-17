//! Plan a reap after mutating the persona store.
use crate::persona::reap_targets::collect_reap_targets;
use crate::{PersonaRuntimeState, PersonaStatus, ReapPersonaRequest};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

/// Store mutation outcome used to publish reap events after releasing the lock.
///
/// # Examples
/// ```
/// use codetether_cognition_core::persona::lifecycle::ReapPlan;
/// let plan = ReapPlan { reaped_ids: vec!["p1".into()], cascade: false,
///     now: chrono::Utc::now(), reason: "manual_reap".into() };
/// assert_eq!(plan.reaped_ids.len(), 1);
/// ```
pub struct ReapPlan {
    /// Personas selected and marked reaped, in publication order.
    pub reaped_ids: Vec<String>,
    /// Whether descendant reaping was requested.
    pub cascade: bool,
    /// Timestamp shared by all resulting events.
    pub now: DateTime<Utc>,
    /// Reason copied from the request or the default.
    pub reason: String,
}

/// Validate a target and mark its selected descendants reaped.
///
/// # Arguments
/// * `personas` - Shared persona store.
/// * `persona_id` - Requested persona identifier.
/// * `req` - Validated or pending lifecycle request.
///
/// # Returns
/// A publication plan after updating the store and releasing its lock.
///
/// # Errors
/// Returns an error when the target persona is unknown.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::persona::lineage::Inherited;
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example( personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>, persona_id: &str, req: ReapPersonaRequest, ) {
/// let _result = codetether_cognition_core::persona::lifecycle::prepare_reap(personas, persona_id, req).await;
/// # }
/// ```
pub async fn prepare_reap(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    persona_id: &str,
    req: ReapPersonaRequest,
) -> Result<ReapPlan> {
    let cascade = req.cascade.unwrap_or(false);
    let now = Utc::now();

    let mut personas = personas.write().await;
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
    Ok(ReapPlan {
        reaped_ids,
        cascade,
        now,
        reason,
    })
}

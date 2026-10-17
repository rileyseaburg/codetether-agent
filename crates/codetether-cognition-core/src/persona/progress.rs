//! Marking persona progress so active work delays idle reaping.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::PersonaRuntimeState;

/// Record that `persona_id` made meaningful progress just now.
///
/// # Arguments
/// * `personas` - Mutable or shared persona records.
/// * `persona_id` - Persona whose progress or tools are requested.
///
/// # Returns
/// Updates the matching persona if it exists.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::{*, execution::*, proposal::dispatch::ExecuteCtx};
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example( personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>, persona_id: &str, ) {
/// let _result = codetether_cognition_core::persona::progress::mark_progress(personas, persona_id).await;
/// # }
/// ```
pub async fn mark_progress(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    persona_id: &str,
) {
    let mut map = personas.write().await;
    if let Some(persona) = map.get_mut(persona_id) {
        persona.last_progress_at = Utc::now();
    }
}

/// Read a persona's allowed tool list, empty when unknown.
///
/// # Arguments
/// * `personas` - Mutable or shared persona records.
/// * `persona_id` - Persona whose progress or tools are requested.
///
/// # Returns
/// A copied allowlist, or an empty list for an unknown persona.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::{*, execution::*, proposal::dispatch::ExecuteCtx};
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example( personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>, persona_id: &str, ) {
/// let _result = codetether_cognition_core::persona::progress::allowed_tools(personas, persona_id).await;
/// # }
/// ```
pub async fn allowed_tools(
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

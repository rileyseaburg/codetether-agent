//! Convert a child-spawn request into a create request with its parent.
use crate::{CreatePersonaRequest, SpawnPersonaRequest};

/// Preserve child request fields and attach the selected parent.
///
/// # Arguments
/// * `parent_id` - Parent of the child persona.
/// * `req` - Validated or pending lifecycle request.
///
/// # Returns
/// The constructed lifecycle record or event.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::persona::lineage::Inherited;
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example(parent_id: &str, req: SpawnPersonaRequest) {
/// let _result = codetether_cognition_core::persona::lifecycle::child_request(parent_id, req);
/// # }
/// ```
pub fn child_request(parent_id: &str, req: SpawnPersonaRequest) -> CreatePersonaRequest {
    CreatePersonaRequest {
        persona_id: req.persona_id,
        name: req.name,
        role: req.role,
        charter: req.charter,
        swarm_id: req.swarm_id,
        parent_id: Some(parent_id.to_string()),
        policy: req.policy,
        tags: Vec::new(),
    }
}

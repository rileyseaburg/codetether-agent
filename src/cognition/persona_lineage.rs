//! Lineage inheritance and branching limits for new personas.

use anyhow::{Result, anyhow};
use std::collections::HashMap;

use super::{PersonaPolicy, PersonaRuntimeState, PersonaStatus};

/// Lineage facts inherited from a parent persona.
#[derive(Default)]
pub(super) struct Inherited {
    pub swarm_id: Option<String>,
    pub depth: u32,
    pub policy: Option<PersonaPolicy>,
}

/// Resolve inherited lineage for `parent_id`, enforcing branching limits.
///
/// # Errors
///
/// Returns an error when the parent is missing, reaped, or already at its
/// branching limit.
pub(super) fn inherit_from_parent(
    personas: &HashMap<String, PersonaRuntimeState>,
    parent_id: &str,
) -> Result<Inherited> {
    let parent = personas
        .get(parent_id)
        .ok_or_else(|| anyhow!("Parent persona not found: {parent_id}"))?;
    if parent.status == PersonaStatus::Reaped {
        return Err(anyhow!("Parent persona {parent_id} is reaped"));
    }

    let branch_limit = parent.policy.max_branching_factor;
    if live_child_count(personas, parent_id) as u32 >= branch_limit {
        return Err(anyhow!(
            "Parent {parent_id} reached branching limit {branch_limit}"
        ));
    }
    Ok(Inherited {
        swarm_id: parent.identity.swarm_id.clone(),
        depth: parent.identity.depth.saturating_add(1),
        policy: Some(parent.policy.clone()),
    })
}

/// Count children of `parent_id` that have not been reaped.
fn live_child_count(personas: &HashMap<String, PersonaRuntimeState>, parent_id: &str) -> usize {
    personas
        .values()
        .filter(|p| {
            p.identity.parent_id.as_deref() == Some(parent_id) && p.status != PersonaStatus::Reaped
        })
        .count()
}

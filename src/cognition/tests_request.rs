//! Request builders for cognition tests.

use super::{CreatePersonaRequest, SpawnPersonaRequest};

/// A minimal create request with the given id, role, and charter.
pub(super) fn create(id: &str, role: &str, charter: &str) -> CreatePersonaRequest {
    CreatePersonaRequest {
        persona_id: Some(id.to_string()),
        name: id.to_string(),
        role: role.to_string(),
        charter: charter.to_string(),
        swarm_id: None,
        parent_id: None,
        policy: None,
        tags: Vec::new(),
    }
}

/// A create request that also sets `swarm_id`.
pub(super) fn create_in_swarm(
    id: &str,
    role: &str,
    charter: &str,
    swarm_id: &str,
) -> CreatePersonaRequest {
    CreatePersonaRequest {
        swarm_id: Some(swarm_id.to_string()),
        ..create(id, role, charter)
    }
}

/// A minimal spawn request with the given id, role, and charter.
pub(super) fn spawn(id: &str, role: &str, charter: &str) -> SpawnPersonaRequest {
    SpawnPersonaRequest {
        persona_id: Some(id.to_string()),
        name: id.to_string(),
        role: role.to_string(),
        charter: charter.to_string(),
        swarm_id: None,
        policy: None,
    }
}

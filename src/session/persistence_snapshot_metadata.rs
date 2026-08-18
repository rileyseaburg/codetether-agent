//! Persisted metadata normalization for borrowed snapshots.

use crate::session::{Session, SessionMetadata};

pub(super) fn normalized(session: &Session) -> SessionMetadata {
    let mut metadata = session.metadata.clone();
    if let Some(identity) = crate::provenance::runtime_agent_identity()
        && let Some(provenance) = metadata.provenance.as_mut()
    {
        provenance.identity.agent_identity_id = Some(identity);
    }
    metadata
}

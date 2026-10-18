//! Persisted metadata normalization for borrowed snapshots.

use codetether_session_metadata::SessionMetadata;

pub(super) fn normalized(source: &SessionMetadata) -> SessionMetadata {
    let mut metadata = (*source).clone();
    if let Some(identity) = codetether_provenance::runtime_agent_identity()
        && let Some(provenance) = metadata.provenance.as_mut()
    {
        provenance.identity.agent_identity_id = Some(identity);
    }
    metadata
}

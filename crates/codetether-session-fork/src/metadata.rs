//! Metadata inherited by an independent edited-conversation branch.
use codetether_session_metadata::SessionMetadata;
pub(super) fn branch_metadata(source: &SessionMetadata, id: &str, agent: &str) -> SessionMetadata {
    let mut metadata = source.clone();
    metadata.run_checkpoint = None;
    metadata.prior_context_turn_allowed = None;
    metadata.knowledge_snapshot = None;
    metadata.shared = false;
    metadata.share_url = None;
    metadata.delegation = Default::default();
    metadata.provenance = Some(codetether_provenance::ExecutionProvenance::for_session(
        id, agent,
    ));
    metadata
}

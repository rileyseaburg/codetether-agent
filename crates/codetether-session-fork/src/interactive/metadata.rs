//! Metadata fields inherited by the existing interactive fork command.
use codetether_session_metadata::SessionMetadata;
pub(super) fn copy(source: &SessionMetadata, target: &mut SessionMetadata) {
    target.auto_apply_edits = source.auto_apply_edits;
    target.allow_network = source.allow_network;
    target.slash_autocomplete = source.slash_autocomplete;
    target.use_worktree = source.use_worktree;
    target.model = source.model.clone();
    target.rlm = source.rlm.clone();
    target.context_policy = source.context_policy;
    target.delegation = source.delegation.clone();
    target.history_sink = source.history_sink.clone();
}

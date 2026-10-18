//! Native session adapter for shared automatic edit confirmation policy.
/// Return whether session or environment policy enables automatic confirmation.
pub fn auto_apply_enabled(meta: &codetether_session_metadata::SessionMetadata) -> bool {
    codetether_edit_confirmation::auto_apply_enabled(meta.auto_apply_edits)
}

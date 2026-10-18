//! Selected preference inheritance for a fresh session.
use crate::{Session, View};
pub(super) fn copy<S: Session, V: View>(view: &V, session: &S, new_session: &mut S) {
    let preferences = view.preferences();
    let metadata = new_session.metadata_mut();
    metadata.auto_apply_edits = preferences.auto_apply_edits;
    metadata.allow_network = preferences.allow_network;
    metadata.slash_autocomplete = preferences.slash_autocomplete;
    metadata.use_worktree = preferences.use_worktree;
    let model = &session.metadata().model;
    metadata.model.clone_from(model);
}

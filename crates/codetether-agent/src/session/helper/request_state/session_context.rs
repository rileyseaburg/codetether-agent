//! Session-owned inputs for provider-step state construction.

use std::path::PathBuf;

use crate::session::Session;

/// Resolve the workspace and effective prior-context policy for a session.
pub(super) fn resolve(session: &Session) -> (PathBuf, bool, bool) {
    let cwd = session
        .metadata
        .directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let allowed = super::super::runtime::prior_context_allowed_for_session(session);
    let autonomous = session.metadata.inherited_prior_context_allowed.is_some();
    (cwd, allowed, autonomous)
}

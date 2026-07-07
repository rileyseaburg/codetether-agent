//! `--yolo` full-auto startup policy.
//!
//! Splits the two concerns so the caller can apply them in the correct order:
//! the effective access mode must be resolved **before** config is applied,
//! while the session edit-application flag is set **after** config so it is
//! never clobbered by [`Session::apply_config`].

use crate::config::AccessMode;
use crate::session::Session;

/// Resolve the effective access mode, letting `--yolo` force
/// [`AccessMode::Full`]. Pure function — no session mutation.
pub(super) fn effective_access_mode(
    access_mode: Option<AccessMode>,
    yolo: bool,
) -> Option<AccessMode> {
    if yolo {
        Some(AccessMode::Full)
    } else {
        access_mode
    }
}

/// Apply the `--yolo` session policy (auto-apply edits). Call this **after**
/// config so the flag is authoritative.
pub(super) fn apply_session_policy(session: &mut Session, yolo: bool) {
    if yolo {
        session.metadata.auto_apply_edits = true;
    }
}

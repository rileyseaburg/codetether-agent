//! `--yolo` full-auto startup policy.
//!
//! Resolves the effective access mode and session edit-application policy when
//! the user launches the TUI with the `--yolo` flag.

use crate::config::AccessMode;
use crate::session::Session;

/// Applies full-auto (`--yolo`) policy to the session and access mode.
///
/// When `yolo` is `true`, edits are auto-applied on the session and the
/// effective access mode is forced to [`AccessMode::Full`]. Otherwise the
/// provided `access_mode` is returned unchanged.
pub(super) fn apply(
    session: &mut Session,
    access_mode: Option<AccessMode>,
    yolo: bool,
) -> Option<AccessMode> {
    if yolo {
        session.metadata.auto_apply_edits = true;
        Some(AccessMode::Full)
    } else {
        access_mode
    }
}

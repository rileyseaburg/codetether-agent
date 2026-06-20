//! Resolve the most-recently-referenced file path from the transcript.

use std::path::{Path, PathBuf};

use crate::tui::app::state::App;

use super::extract::path_from_message;

/// Absolute path of the most recently referenced file, if any.
///
/// Scans the transcript newest-first for a shared `File` message or a
/// file-oriented tool call (`read`/`write`/`edit`/…), returning the first
/// resolved path that exists on disk.
pub(super) fn latest_shared_file(app: &App, workspace_dir: &Path) -> Option<PathBuf> {
    app.state
        .messages
        .iter()
        .rev()
        .filter_map(|m| path_from_message(m, workspace_dir))
        .find(|p| p.is_file())
}

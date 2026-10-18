//! Test binding for shared transcript file-reference resolution.
use crate::tui::app::state::App;
use std::path::{Path, PathBuf};
pub(super) fn latest_shared_file(app: &App, workspace_dir: &Path) -> Option<PathBuf> {
    codetether_chat_files::latest_shared_file(&app.state.messages, workspace_dir)
}

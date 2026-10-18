//! Native composer and picker bindings for shared file attachment commands.
use crate::tui::app::state::App;
use crate::tui::constants::FILE_SHARE_MAX_BYTES;
use std::path::Path;
mod view;

pub fn attach_file_to_input(app: &mut App, workspace_dir: &Path, path: &Path) {
    codetether_file_share::interactive::attach(
        &mut view::Composer(app),
        workspace_dir,
        path,
        FILE_SHARE_MAX_BYTES,
    );
}

pub(crate) fn command(app: &mut App, cwd: &Path, cleaned: &str) {
    codetether_file_share::interactive::command(
        &mut view::Composer(app),
        cwd,
        cleaned,
        FILE_SHARE_MAX_BYTES,
    );
}

//! Native command entry for shared embedded editor opening.
use crate::tui::app::state::App;
use std::path::Path;
pub(super) fn open_editor(app: &mut App, cwd: &Path, arg: &str) {
    crate::tui::app::fuzzy_find::open_argument(app, cwd, arg);
}

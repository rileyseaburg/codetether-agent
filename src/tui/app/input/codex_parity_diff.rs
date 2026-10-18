//! Native adapter for the shared working-tree report.
use crate::tui::app::state::App;
use std::path::Path;
#[path = "codex_parity_diff_render.rs"]
mod render;
pub(super) fn show(app: &mut App, cwd: &Path) {
    render::finish(app, codetether_git_view::working_tree_report(cwd));
}

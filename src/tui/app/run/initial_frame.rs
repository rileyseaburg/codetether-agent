//! Immediate loading-frame rendering for TUI startup.

use std::path::Path;

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::tui::app::safe_draw::draw_ui;
use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;

/// Creates application state and renders it before slow startup work begins.
///
/// # Arguments
///
/// * `terminal` — Active terminal receiving the first frame.
/// * `cwd` — Workspace path displayed while startup runs.
/// * `allow_network` — Requested network policy copied into application state.
///
/// # Returns
///
/// The initialized application state reused by the event loop.
///
/// # Errors
///
/// Returns an error when the terminal cannot render the loading frame.
///
/// # Examples
///
/// ```text
/// Loading providers and workspace...
/// ```
pub(super) fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    cwd: &Path,
    allow_network: bool,
) -> anyhow::Result<App> {
    let mut app = App::default();
    super::hydrate_initial::loading(&mut app, cwd, allow_network);
    let view = SessionView {
        directory: Some(cwd.to_path_buf()),
        ..SessionView::default()
    };
    draw_ui(terminal, &mut app, &view)?;
    Ok(app)
}

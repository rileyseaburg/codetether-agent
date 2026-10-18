//! Immediate loading-frame construction before startup dependencies run.
use crate::Terminal;
use codetether_tui_models::SessionView;
use std::path::Path;
/// Native frontend construction and rendering operations.
/// # Implementors
/// Frontends retain their App and provenance principal types and drawing backend.
pub trait Host {
    /// Frontend initialized for the loading frame.
    type App: Default;
    /// Default principal used before a session is resolved.
    type Principal: Default;
    /// Populate loading fields on the new frontend.
    fn loading(app: &mut Self::App, cwd: &Path, allow_network: bool);
    /// Render using the existing native drawing path.
    /// # Errors
    /// Returns terminal drawing failures.
    fn draw(
        terminal: &mut Terminal,
        app: &mut Self::App,
        view: &SessionView<Self::Principal>,
    ) -> anyhow::Result<bool>;
}
/// Construct the frontend and draw its first loading frame.
/// # Arguments
/// Receives the initialized terminal, workspace path and requested network policy.
/// # Returns
/// The frontend reused by the main event loop.
/// # Errors
/// Returns the native drawing error without starting later startup work.
pub fn draw<H: Host>(
    terminal: &mut Terminal,
    cwd: &Path,
    allow_network: bool,
) -> anyhow::Result<H::App> {
    let mut app = H::App::default();
    H::loading(&mut app, cwd, allow_network);
    let view = SessionView::<H::Principal> {
        directory: Some(cwd.to_path_buf()),
        ..SessionView::default()
    };
    H::draw(terminal, &mut app, &view)?;
    Ok(app)
}

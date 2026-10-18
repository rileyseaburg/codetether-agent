//! Native loading-state and drawing bindings for the first frame.
use super::*;
use crate::tui::app::session_runtime::SessionView;
impl codetether_tui_startup::first_frame::Host for Native {
    type App = App;
    type Principal = crate::provenance::RuntimePrincipal;
    fn loading(app: &mut App, cwd: &Path, allow_network: bool) {
        hydrate_initial::loading(app, cwd, allow_network);
    }
    fn draw(terminal: &mut Terminal, app: &mut App, view: &SessionView) -> anyhow::Result<bool> {
        crate::tui::app::safe_draw::draw_ui(terminal, app, view)
    }
}

//! Native first-frame hydration entry and regression coverage.
use crate::{session::Session, tui::app::state::App};
use std::path::Path;
pub(super) fn initial(
    app: &mut App,
    cwd: &Path,
    allow_network: bool,
    peer_ready: bool,
    session: &Session,
) {
    codetether_tui_startup::hydrate::initial(
        &mut super::hydrate_host::Native(app),
        cwd,
        allow_network,
        peer_ready,
        session,
    );
}
pub(super) fn loading(app: &mut App, cwd: &Path, allow_network: bool) {
    codetether_tui_startup::hydrate::loading(
        &mut super::hydrate_host::Native(app),
        cwd,
        allow_network,
    );
}
#[cfg(test)]
#[path = "hydrate_initial_tests.rs"]
mod tests;

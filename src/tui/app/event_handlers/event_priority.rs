//! Native bindings for shared interrupt and modal priority.
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
use crossterm::event::KeyEvent;
use std::path::Path;
mod host;
pub(super) async fn handle(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> Option<bool> {
    codetether_tui_navigation::events::priority::handle(
        &mut host::Native {
            app,
            cwd,
            slot,
            runtime,
        },
        key,
    )
    .await
}

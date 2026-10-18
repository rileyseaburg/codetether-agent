//! Bind goal-draft keys to native editor/session state.
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
use crossterm::event::KeyEvent;
pub(crate) async fn handle(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> bool {
    codetether_goal_editor::handle(
        &mut super::host::Runtime {
            editor: super::host::Editor { app },
            slot,
            runtime,
        },
        key,
    )
    .await
}

//! Existing interrupt and modal priorities, independent of chat submission.

use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
use crossterm::event::KeyEvent;
use std::path::Path;

pub(super) async fn handle(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> Option<bool> {
    if let Some(quit) = super::interrupt_key::handle(app, runtime, key) {
        return Some(quit);
    }
    if super::interlude_key::handle(app, key)
        || super::goal_prompt_key::handle_goal_prompt_key(app, key)
    {
        return Some(false);
    }
    if app.state.spawn_form.is_some()
        && crate::tui::app::spawn_form::handle_spawn_form_key(app, cwd, slot, key).await
    {
        return Some(false);
    }
    if super::fuzzy_find_key::handle_fuzzy_find_key(app, cwd, key) {
        return Some(false);
    }
    if super::editor_lsp_key::handle_editor_lsp_key(app, cwd, key).await {
        return Some(false);
    }
    if super::editor_key::handle_editor_key(app, cwd, key) {
        return Some(false);
    }
    None
}

//! Route draft keys before approval, answer-review, or disk-file saving.

use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
use crate::tui::ui::editor::{EditorInput, apply::apply, map_key};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) async fn handle(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> bool {
    if app.state.goal_editor.is_none() {
        return false;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('c' | 'q'))
    {
        return false;
    }
    let Some(action) = map_key(key) else {
        return true;
    };
    match action {
        EditorInput::Save => {
            if let Err(error) = super::save::save(app, slot, runtime).await {
                app.state.status = format!("Goal not saved: {error}");
            }
        }
        EditorInput::Quit => {
            super::close(app);
            app.state.status = "Goal edit discarded; live goal unchanged".into();
        }
        EditorInput::OpenFinder => {
            app.state.status = "Editing a goal, not a file — Ctrl+S save · Esc discard".into();
        }
        edit => {
            if let Some(buffer) = app.state.editor.as_mut()
                && let Err(error) = apply(buffer, edit)
            {
                app.state.status = format!("Goal editor: {error}");
            }
        }
    }
    app.state.needs_redraw = true;
    true
}

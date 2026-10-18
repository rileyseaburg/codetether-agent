//! Route goal-draft keys before unrelated approval and file-edit behavior.
use crate::{Read, Session, Write};
use codetether_editor::{EditorInput, apply::apply, map_key};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Apply one key to the active goal editor before general TUI key handling.
/// # Arguments
/// Receives session-aware editor access and the terminal key event.
/// # Returns
/// Whether the key was consumed; quit shortcuts remain available to the caller.
/// # Examples
/// ```no_run
/// async fn key<S: codetether_goal_editor::Session>(session: &mut S, key: crossterm::event::KeyEvent) {
///     codetether_goal_editor::handle(session, key).await;
/// }
/// ```
pub async fn handle<S: Session>(session: &mut S, key: KeyEvent) -> bool {
    if session.editor().draft().is_none() {
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
            if let Err(error) = super::save::save(session).await {
                session
                    .editor_mut()
                    .status(format!("Goal not saved: {error}"));
            }
        }
        EditorInput::Quit => {
            super::close(session.editor_mut());
            session
                .editor_mut()
                .status("Goal edit discarded; live goal unchanged".into());
        }
        EditorInput::OpenFinder => {
            session
                .editor_mut()
                .status("Editing a goal, not a file — Ctrl+S save · Esc discard".into());
        }
        edit => {
            if let Some(buffer) = session.editor_mut().buffer_mut()
                && let Err(error) = apply(buffer, edit)
            {
                session.editor_mut().status(format!("Goal editor: {error}"));
            }
        }
    }
    session.editor_mut().redraw();
    true
}

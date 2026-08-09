//! Save and cancel handling for an approval-backed editor session.

use crate::tui::app::state::App;
use crate::tui::ui::editor::EditorInput;

pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, action: &EditorInput) -> bool {
    match action {
        EditorInput::Save => save(app),
        EditorInput::Quit => cancel(app),
        EditorInput::OpenFinder => {
            app.state.status =
                "File switching is disabled while revising an approval proposal".into();
        }
        _ => return false,
    }
    app.state.needs_redraw = true;
    true
}

fn save(app: &mut App) {
    let Some(text) = app.state.editor.as_ref().map(|buffer| buffer.text()) else {
        return cancel(app);
    };
    let Some(mut session) = app.state.approval_edit.take() else {
        return;
    };
    if session.store(text) {
        let buffer = session.buffer();
        let path = buffer.path().display().to_string();
        let (current, total) = session.progress();
        app.state.editor = Some(buffer);
        app.state.approval_edit = Some(session);
        app.state.editor_scroll = 0;
        app.state.editor_hscroll = 0;
        app.state.status = format!(
            "Editing proposed file {current}/{total}: {path} · Ctrl+S next/apply · Esc cancel"
        );
        return;
    }
    let patch = session.revised_patch();
    match super::finish::apply(&session.id, &patch) {
        Ok(status) => super::close::close(app, status),
        Err(error) => {
            app.state.approval_edit = Some(session);
            app.state.status = format!("Could not apply edited proposal: {error}");
        }
    }
}

fn cancel(app: &mut App) {
    super::close::close(
        app,
        "Approval edit cancelled; original proposal is still pending".into(),
    );
}

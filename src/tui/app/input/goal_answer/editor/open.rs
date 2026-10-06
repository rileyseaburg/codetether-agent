//! Open and close a virtual editor without saving files or mutating the goal.

use super::Draft;
use crate::session::tasks::runtime::answer_review;
use crate::tui::{app::state::App, models::ViewMode, ui::editor::FileBuffer};

pub(crate) fn open(app: &mut App, id: &str) {
    if app.state.editor.is_some() || app.state.approval_edit.is_some() {
        app.state.status = "Close the current editor before editing the goal".into();
        return;
    }
    match answer_review::read(id) {
        Ok(state) => {
            let Some(goal) = state.goal else {
                app.state.status = "No goal to edit; use /goal set <objective> first".into();
                return;
            };
            app.state.editor = Some(FileBuffer::draft("Session goal (draft)", &goal.objective));
            app.state.goal_editor = Some(Draft {
                session_id: id.into(),
                original: goal,
            });
            app.state.editor_scroll = 0;
            app.state.editor_hscroll = 0;
            app.state.editor_lsp.clear_hover();
            app.state.clear_input();
            app.state.set_view_mode(ViewMode::Editor);
            app.state.status =
                "Editing current goal — Ctrl+S save · Esc discard · Enter newline".into();
            app.state.needs_redraw = true;
        }
        Err(error) => app.state.status = format!("Cannot open goal editor: {error}"),
    }
}

pub(crate) fn close(app: &mut App) {
    app.state.goal_editor = None;
    app.state.editor = None;
    app.state.editor_scroll = 0;
    app.state.editor_hscroll = 0;
    app.state.editor_lsp.clear_hover();
    app.state.set_view_mode(ViewMode::Chat);
    app.state.needs_redraw = true;
}

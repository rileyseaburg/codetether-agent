//! Opens a pending patch proposal in the in-TUI source editor.

use std::path::Path;

use crate::tui::app::state::{App, approval_queue};
use crate::tui::models::ViewMode;

use approval_queue::edit_session::ApprovalEditSession;

pub(in crate::tui::app::event_handlers) fn open(app: &mut App, cwd: &Path) -> bool {
    let Some(item) = approval_queue::active() else {
        return false;
    };
    if item.tool != "apply_patch" {
        app.state.status = "Only patch approvals can be opened in the code editor".into();
        return true;
    }
    let Some(patch) = item.preview.as_deref() else {
        app.state.status = "This approval has no editable patch preview".into();
        return true;
    };
    match ApprovalEditSession::from_patch(cwd, item.id, patch) {
        Ok(session) => activate(app, session),
        Err(error) => app.state.status = format!("Cannot edit approval patch: {error}"),
    }
    true
}

fn activate(app: &mut App, session: ApprovalEditSession) {
    let buffer = session.buffer();
    let (current, total) = session.progress();
    let path = buffer.path().display().to_string();
    app.state.editor = Some(buffer);
    app.state.approval_edit = Some(session);
    app.state.editor_scroll = 0;
    app.state.editor_hscroll = 0;
    app.state.set_view_mode(ViewMode::Editor);
    app.state.status =
        format!("Editing proposed file {current}/{total}: {path} · Ctrl+S next/apply · Esc cancel");
}

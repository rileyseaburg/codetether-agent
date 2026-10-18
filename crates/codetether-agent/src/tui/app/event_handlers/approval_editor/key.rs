//! Dispatch approval-editor input through the crate-owned host.
use crate::tui::app::state::App;
use codetether_editor::EditorInput;

pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, action: &EditorInput) -> bool {
    codetether_approval_edit::interactive::handle(&mut super::borrow::host(&mut app.state), action)
}

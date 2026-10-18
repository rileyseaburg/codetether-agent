//! Bind native approval editor actions to the shared workflow.
use crate::tui::app::state::App;
use codetether_editor::EditorInput;
pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, action: &EditorInput) -> bool {
    codetether_approval_edit::interactive::handle(&mut super::host::Native(app), action)
}

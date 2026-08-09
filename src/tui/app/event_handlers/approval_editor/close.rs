//! Returns an approval editor session to the chat view.

use crate::tui::app::state::{App, approval_queue};
use crate::tui::models::ViewMode;

pub(super) fn close(app: &mut App, status: String) {
    app.state.approval_edit = None;
    app.state.editor = None;
    app.state.editor_scroll = 0;
    app.state.editor_hscroll = 0;
    app.state.set_view_mode(ViewMode::Chat);
    app.state.approval_waiting = approval_queue::active().is_some();
    app.state.status = status;
}

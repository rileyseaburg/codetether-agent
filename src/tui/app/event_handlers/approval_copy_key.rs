//! Clipboard handling for approval preview content.

use crate::tui::app::state::{App, approval_queue};

pub(in crate::tui::app::event_handlers) fn copy_preview(app: &mut App) -> bool {
    let Some(preview) = approval_queue::active().and_then(|item| item.preview) else {
        return false;
    };
    app.state.status = match crate::tui::clipboard_text::copy_text(&preview) {
        Ok(method) => format!("Copied clean approval preview ({method})"),
        Err(error) => format!("Could not copy approval preview: {error}"),
    };
    true
}

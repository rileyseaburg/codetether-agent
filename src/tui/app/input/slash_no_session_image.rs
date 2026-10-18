//! Shared image attachment behavior for session-free slash commands.
use crate::tui::app::state::App;
use std::path::Path;
pub(super) fn attach_image_command(app: &mut App, cwd: &Path, cleaned: &str) {
    crate::tui::app::input::attach_image_command(
        app,
        cwd,
        cleaned,
        codetether_image::interactive::Presentation::Pending,
    );
}

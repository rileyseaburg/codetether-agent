//! Helpers for [`super::slash_no_session`]: file attach and model picker.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::state::App;

/// Attach a file to the input, or open the file picker when `cleaned` is empty.
pub(super) fn attach_file_command(app: &mut App, cwd: &Path, cleaned: &str) {
    crate::tui::app::file_share::command(app, cwd, cleaned);
}
/// Open the shared model picker without a session.
pub(super) fn open_model_view(app: &mut App, registry: &Option<Arc<ProviderRegistry>>) {
    crate::tui::app::model_picker::open_without_session(app, registry.as_ref());
}

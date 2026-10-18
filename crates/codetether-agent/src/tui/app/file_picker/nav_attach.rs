//! Bind selected-file attachment to native composer state.
use crate::tui::app::state::App;
pub fn file_picker_attach(app: &mut App) {
    codetether_file_picker::interactive::file_picker_attach(&mut super::host::Native(app));
}

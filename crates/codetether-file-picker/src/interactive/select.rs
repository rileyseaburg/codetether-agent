use super::View;

use super::attach::attach_path;

/// Apply the file-picker workflow through frontend state bindings.
/// # Arguments
/// Receives the frontend view and relevant workspace/file path, when required.
/// # Returns
/// Updates picker state, view selection and attachment presentation.
pub fn file_picker_attach(view: &mut impl View) {
    let path = preview_path(view)
        .filter(|p| p.is_file())
        .or_else(|| selected_path(view));
    if let Some(path) = path.filter(|path| path.is_file()) {
        attach_path(view, &path);
    }
}

fn preview_path(view: &mut impl View) -> Option<std::path::PathBuf> {
    view.picker()
        .preview
        .as_ref()
        .map(|preview| preview.path.clone())
}

fn selected_path(view: &mut impl View) -> Option<std::path::PathBuf> {
    let picker = view.picker();
    picker
        .entries
        .get(picker.selected)
        .map(|entry| entry.path.clone())
}

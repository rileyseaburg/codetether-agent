use std::path::Path;

use super::View;

use crate::is_image_file;

/// Apply the file-picker workflow through frontend state bindings.
/// # Arguments
/// Receives the frontend view and relevant workspace/file path, when required.
/// # Returns
/// Updates picker state, view selection and attachment presentation.
pub fn attach_path(view: &mut impl View, path: &Path) {
    if is_image_file(path) {
        attach_image(view, path);
    } else {
        let workspace = view.picker().workspace_dir.clone();
        view.attach_file(&workspace, path);
    }
    view.picker().active = false;
    view.show_chat();
}

fn attach_image(view: &mut impl View, path: &Path) {
    match codetether_image::attachment_from_file(path) {
        Ok(attachment) => {
            view.push_image(attachment);
            view.set_status(format!("Attached image {}.", path.display()));
            view.push_system(format!(
                "Image attached: {}. Type a message and press Enter.",
                path.display()
            ));
        }
        Err(msg) => {
            view.set_status("Failed to attach image".to_string());
            view.push_error(msg);
        }
    }
    view.scroll_to_bottom();
}

//! Image-attach helper for [`super::slash_no_session`].

use std::path::Path;

use crate::tui::app::state::App;

/// Attach an image file to the pending-images queue for the next send.
///
/// Resolves `cleaned` against `cwd` when relative and pushes the decoded
/// attachment onto `app.state.pending_images`. Reports usage when empty.
pub(super) fn attach_image_command(app: &mut App, cwd: &Path, cleaned: &str) {
    if cleaned.is_empty() {
        app.state.status =
            "Usage: /image <path> (png, jpg, jpeg, gif, webp, bmp, svg).".to_string();
        return;
    }
    let path = Path::new(cleaned);
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };
    match crate::tui::app::input::attach_image_file(&resolved) {
        Ok(attachment) => {
            app.state.pending_images.push(attachment);
            let count = app.state.pending_images.len();
            app.state.status = format!(
                "📷 Attached {}. {count} image(s) pending.",
                resolved.display()
            );
        }
        Err(msg) => app.state.status = format!("Failed to attach image: {msg}"),
    }
}

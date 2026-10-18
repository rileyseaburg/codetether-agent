//! Image command path resolution, queueing and presentation.
use super::{Presentation, View};
use std::path::Path;
/// Attach an image using the existing chat or pending-input presentation.
/// # Arguments
/// * `view` - Pending queue and command output bindings.
/// * `cwd` - Base directory for relative paths.
/// * `cleaned` - Path after caller-side command whitespace/quote trimming.
/// * `mode` - Chat notices or status-only pending-input output.
/// # Returns
/// Completes after queueing an image or displaying usage/read errors.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_image::interactive::View>(view: &mut V) {
/// codetether_image::interactive::attach(view, std::path::Path::new("."), "photo.png", codetether_image::interactive::Presentation::Chat);
/// # }
/// ```
pub fn attach<V: View>(view: &mut V, cwd: &Path, cleaned: &str, mode: Presentation) {
    if cleaned.is_empty() {
        view.set_status("Usage: /image <path> (png, jpg, jpeg, gif, webp, bmp, svg).".to_string());
        return;
    }
    let path = Path::new(cleaned);
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };
    match crate::attachment_from_file(&resolved) {
        Ok(attachment) => {
            let display = resolved.display();
            let count = view.push_image(attachment);
            match mode {
                Presentation::Chat => {
                    view.set_status(format!(
                        "📷 Attached {display}. {count} image(s) pending. Press Enter to send."
                    ));
                    view.push_system_message(format!(
                        "📷 Image attached: {display}. Type a message and press Enter to send."
                    ));
                }
                Presentation::Pending => {
                    view.set_status(format!("📷 Attached {display}. {count} image(s) pending."))
                }
            }
        }
        Err(msg) => match mode {
            Presentation::Chat => {
                view.push_system_message(format!("Failed to attach image: {msg}"))
            }
            Presentation::Pending => view.set_status(format!("Failed to attach image: {msg}")),
        },
    }
}

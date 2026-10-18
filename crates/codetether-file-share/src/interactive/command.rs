//! Empty-path picker dispatch and explicit file attachment.
use super::{View, attach};
use std::path::Path;
/// Open the picker or attach the requested file to the composer.
/// # Arguments
/// * `view` - Composer and picker bindings.
/// * `cwd` - Working directory for relative paths.
/// * `cleaned` - Command path after whitespace/quote trimming.
/// * `max_bytes` - File content limit used by the existing snippet builder.
/// # Returns
/// Completes after opening the picker or showing the attachment result.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_file_share::interactive::View>(view: &mut V) {
/// codetether_file_share::interactive::command(view, std::path::Path::new("."), "", 65536);
/// # }
/// ```
pub fn command<V: View>(view: &mut V, cwd: &Path, cleaned: &str, max_bytes: usize) {
    if cleaned.is_empty() {
        view.open_picker(cwd);
    } else {
        attach(view, cwd, Path::new(cleaned), max_bytes);
    }
}

//! Clipboard handling for approval preview content.

use super::Host;
use codetether_approval_queue as approval_queue;

/// Copy the active clean approval preview.
/// # Arguments
/// * `host` — Frontend state and command bindings.
/// # Returns
/// Whether the action was consumed.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_approval_view::interactive::Host) {
/// let _ = codetether_approval_view::interactive::copy_preview(host);
/// # }
/// ```
pub fn copy_preview(host: &mut impl Host) -> bool {
    let Some(preview) = approval_queue::active().and_then(|item| item.preview) else {
        return false;
    };
    host.set_status(match codetether_terminal_clipboard::copy_text(&preview) {
        Ok(method) => format!("Copied clean approval preview ({method})"),
        Err(error) => format!("Could not copy approval preview: {error}"),
    });
    true
}

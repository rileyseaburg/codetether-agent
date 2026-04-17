//! Attachment badge suffix for the input area title.
//!
//! [`attachment_suffix`] produces a short string like `| 📷 2 attached | 🧭 1 queued`
//! reflecting pending images and steering messages.

use crate::tui::app::state::App;

/// Build a short suffix like `| 📷 2 attached | 🧭 1 queued` or empty.
///
/// Returns an empty `String` when there are no pending images or steering
/// messages, so the input title stays clean.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::attachment::attachment_suffix;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// // With no attachments the suffix is empty.
/// let suffix = attachment_suffix(app);
/// assert!(suffix.is_empty() || suffix.contains("📷"));
/// # }
/// ```
pub fn attachment_suffix(app: &App) -> String {
    let pending_images = app.state.pending_images.len();
    let steering = app.state.steering_count();
    if pending_images == 0 && steering == 0 {
        return String::new();
    }
    let image_part = (pending_images > 0).then(|| format!("📷 {pending_images} attached"));
    let steering_part = (steering > 0).then(|| format!("🧭 {steering} queued"));
    let parts = [image_part, steering_part]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" | ");
    format!(" | {parts} ")
}

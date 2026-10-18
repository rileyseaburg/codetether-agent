//! Terminal view selection and global overlay composition.
//!
//! [render] selects the current view through [Host], then renders overlays in
//! priority order. Implement the small host traits with application adapters.
mod chat;
mod dispatch;
mod host;
mod overlays;
pub use host::{Chat, Diagnostics, Host, Management, Overlays, Primary, State};
use ratatui::Frame;

/// Render the active view followed by global overlays.
/// # Arguments
/// * `frame` - Destination terminal frame.
/// * `host` - Current view and application render bindings.
/// # Returns
/// Draws the selected view and all active overlays.
/// # Examples
/// ```no_run
/// fn draw(frame: &mut ratatui::Frame, host: &mut impl codetether_tui_render::Host) {
///     codetether_tui_render::render(frame, host);
/// }
/// ```
pub fn render(frame: &mut Frame, host: &mut impl Host) {
    dispatch::render(frame, host);
    overlays::render(frame, host);
}

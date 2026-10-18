//! Minimum terminal dimensions for the Webview.
use ratatui::layout::Rect;

/// Minimum terminal size for webview layout.
const MIN_WIDTH: u16 = 90;

const MIN_HEIGHT: u16 = 18;

/// Check whether the terminal is smaller than 90 columns by 18 rows.
/// # Arguments
/// * `area` - Available terminal rectangle.
/// # Returns
/// True if either dimension is below the minimum.
/// # Examples
/// ```
/// assert!(codetether_webview::chat::terminal_too_small(ratatui::layout::Rect::new(0, 0, 40, 10)));
/// ```
pub fn terminal_too_small(area: Rect) -> bool {
    area.width < MIN_WIDTH || area.height < MIN_HEIGHT
}

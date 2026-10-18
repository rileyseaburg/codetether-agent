//! Help centered rect content and presentation.
use ratatui::layout::{Constraint, Direction, Layout, Rect};
/// Compute a centered popup with percentage-based width and height.
/// # Arguments
/// * `percent_x` - Percentage of the available width.
/// * `percent_y` - Percentage of the available height.
/// * `r` - Available screen rectangle.
/// # Returns
/// The centered inner rectangle.
/// # Panics
/// Values above 100 can overflow the original percentage subtraction.
/// # Examples
/// ```
/// let outer = ratatui::layout::Rect::new(0, 0, 100, 100);
/// let inner = codetether_help_view::centered_rect(60, 60, outer);
/// assert_eq!(inner.width, 60);
/// ```
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

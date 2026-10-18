//! Responsive height of the bottom status strip.

/// Height reserved for the bottom status bar, in rows.
///
/// Stacks to 3 rows when the terminal is narrower than the
/// [`crate::STACK_WIDTH_THRESHOLD`]; otherwise a single row.
pub(super) fn status_bar_height(width: u16) -> u16 {
    if width >= crate::STACK_WIDTH_THRESHOLD {
        1
    } else {
        3
    }
}

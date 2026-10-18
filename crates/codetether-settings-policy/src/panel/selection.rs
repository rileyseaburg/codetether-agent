//! Bounded cursor movement for the settings panel's nine rows.

/// Number of actionable settings rows in display order.
pub const COUNT: usize = 9;

/// Move selection up one row without wrapping.
///
/// # Arguments
/// * `index` — Current zero-based settings row.
/// # Returns
/// Updates the index in place, stopping at zero.
/// # Examples
/// ```
/// let mut index = 1;
/// codetether_settings_policy::panel::select_previous(&mut index);
/// assert_eq!(index, 0);
/// ```
pub fn select_previous(index: &mut usize) {
    if *index > 0 {
        *index -= 1;
    }
}

/// Move selection down one row without wrapping.
///
/// # Arguments
/// * `index` — Current zero-based settings row.
/// # Returns
/// Updates the index in place, stopping at the last settings row.
/// # Examples
/// ```
/// let mut index = 8;
/// codetether_settings_policy::panel::select_next(&mut index);
/// assert_eq!(index, 8);
/// ```
pub fn select_next(index: &mut usize) {
    if *index + 1 < COUNT {
        *index += 1;
    }
}

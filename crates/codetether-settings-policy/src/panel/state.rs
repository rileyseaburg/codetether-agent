//! Selection state for the settings panel.

/// Selected settings row and bounded navigation operations.
///
/// Preference values and persistence remain separate from panel selection.
///
/// # Examples
/// ```
/// let mut panel = codetether_settings_policy::panel::PanelState::default();
/// panel.select_previous();
/// assert_eq!(panel.selected, 0);
/// panel.select_next();
/// assert_eq!(panel.selected, 1);
/// ```
#[derive(Default)]
pub struct PanelState {
    /// Zero-based index of the selected settings row.
    pub selected: usize,
}

impl PanelState {
    /// Move to the previous row without wrapping.
    /// # Examples
    /// ```
    /// let mut panel = codetether_settings_policy::panel::PanelState { selected: 1 };
    /// panel.select_previous();
    /// assert_eq!(panel.selected, 0);
    /// ```
    pub fn select_previous(&mut self) {
        super::select_previous(&mut self.selected);
    }

    /// Move to the next row without wrapping past the final row.
    /// # Examples
    /// ```
    /// use codetether_settings_policy::panel::{COUNT, PanelState};
    /// let mut panel = PanelState { selected: COUNT - 1 };
    /// panel.select_next();
    /// assert_eq!(panel.selected, COUNT - 1);
    /// ```
    pub fn select_next(&mut self) {
        super::select_next(&mut self.selected);
    }
}

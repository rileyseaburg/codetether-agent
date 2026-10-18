//! Preference updates and status output borrowed from a frontend.
use super::{Preference, Preferences, View};

/// Interactive preference state paired with its status output.
///
/// # Examples
/// ```
/// use codetether_settings_policy::persistence::{BorrowedView, Preference, Preferences, View};
/// let mut preferences = Preferences::default();
/// let mut status = String::new();
/// let mut view = BorrowedView { preferences: &mut preferences, status: &mut status };
/// view.set_preference(Preference::Network, true);
/// view.set_status("Saved".into());
/// assert!(view.preferences.allow_network);
/// assert_eq!(view.status.as_str(), "Saved");
/// ```
pub struct BorrowedView<'a> {
    /// Editable frontend preferences.
    pub preferences: &'a mut Preferences,
    /// Frontend status line.
    pub status: &'a mut String,
}

impl View for BorrowedView<'_> {
    fn set_preference(&mut self, preference: Preference, enabled: bool) {
        self.preferences.set(preference, enabled);
    }
    fn set_status(&mut self, status: String) {
        *self.status = status;
    }
}

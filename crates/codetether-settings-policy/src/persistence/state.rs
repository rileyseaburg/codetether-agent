//! Interactive preference values and their default policy.
use super::Preference;

/// Preferences displayed and edited by the interactive frontend.
///
/// Persisted session metadata remains separate so callers can control when
/// changes are saved and how persistence failures are reported.
///
/// # Examples
/// ```
/// use codetether_settings_policy::persistence::{Preference, Preferences};
/// let mut preferences = Preferences::default();
/// assert!(!preferences.allow_network);
/// assert!(preferences.slash_autocomplete);
/// preferences.set(Preference::Network, true);
/// assert!(preferences.allow_network);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preferences {
    /// Automatically apply accepted edits.
    pub auto_apply_edits: bool,
    /// Permit network access in sandboxed commands.
    pub allow_network: bool,
    /// Offer slash-command autocomplete.
    pub slash_autocomplete: bool,
    /// Execute isolated work in a worktree.
    pub use_worktree: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            auto_apply_edits: false,
            allow_network: false,
            slash_autocomplete: true,
            use_worktree: true,
        }
    }
}

impl Preferences {
    /// Change the selected preference in memory.
    /// # Arguments
    /// * `preference` — Preference to update.
    /// * `enabled` — New value.
    /// # Returns
    /// Updates the field without saving session metadata.
    /// # Examples
    /// ```
    /// use codetether_settings_policy::persistence::{Preference, Preferences};
    /// let mut preferences = Preferences::default();
    /// preferences.set(Preference::Worktree, false);
    /// assert!(!preferences.use_worktree);
    /// ```
    pub fn set(&mut self, preference: Preference, enabled: bool) {
        match preference {
            Preference::AutoApply => self.auto_apply_edits = enabled,
            Preference::Network => self.allow_network = enabled,
            Preference::Autocomplete => self.slash_autocomplete = enabled,
            Preference::Worktree => self.use_worktree = enabled,
        }
    }
}

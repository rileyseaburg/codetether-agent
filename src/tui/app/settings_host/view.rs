//! Native interactive preference field bindings.
use crate::tui::app::state::App;
use codetether_settings_policy::persistence::Preference;
pub(super) struct Settings<'a>(pub(super) &'a mut App);
impl codetether_settings_policy::persistence::View for Settings<'_> {
    fn set_preference(&mut self, preference: Preference, enabled: bool) {
        match preference {
            Preference::AutoApply => self.0.state.auto_apply_edits = enabled,
            Preference::Network => self.0.state.allow_network = enabled,
            Preference::Autocomplete => self.0.state.slash_autocomplete = enabled,
            Preference::Worktree => self.0.state.use_worktree = enabled,
        }
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}

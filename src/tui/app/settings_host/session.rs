//! Native persisted preference field bindings.
use crate::session::Session;
use codetether_settings_policy::persistence::Preference;
impl codetether_settings_policy::persistence::Session for Session {
    fn set_preference(&mut self, preference: Preference, enabled: bool) {
        match preference {
            Preference::AutoApply => self.metadata.auto_apply_edits = enabled,
            Preference::Network => self.metadata.allow_network = enabled,
            Preference::Autocomplete => self.metadata.slash_autocomplete = enabled,
            Preference::Worktree => self.metadata.use_worktree = enabled,
        }
    }
    async fn save(&self) -> anyhow::Result<()> {
        Session::save(self).await
    }
}

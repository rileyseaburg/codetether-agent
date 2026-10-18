//! Native setting values and existing setters for shared panel dispatch.
use super::{App, Session};
use codetether_settings_policy::{cycles::Cycle, persistence::Preference};

pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub session: &'a mut Session,
}
impl codetether_settings_policy::panel::Host for Native<'_> {
    fn current(&self, preference: Preference) -> bool {
        match preference {
            Preference::AutoApply => self.app.state.preferences.auto_apply_edits,
            Preference::Network => self.app.state.preferences.allow_network,
            Preference::Autocomplete => self.app.state.preferences.slash_autocomplete,
            Preference::Worktree => self.app.state.preferences.use_worktree,
        }
    }
    async fn set_preference(&mut self, preference: Preference, next: bool) {
        match preference {
            Preference::AutoApply => {
                crate::tui::app::commands::set_auto_apply_edits(self.app, self.session, next).await
            }
            Preference::Network => {
                super::super::network::set_network_access(self.app, self.session, next).await
            }
            Preference::Autocomplete => {
                super::super::set_slash_autocomplete(self.app, self.session, next).await
            }
            Preference::Worktree => {
                super::super::set_use_worktree(self.app, self.session, next).await
            }
        }
    }
    async fn cycle_access(&mut self) {
        super::super::access_mode::cycle_access_mode(self.app, self.session).await;
    }
    async fn cycle_provider(&mut self, setting: Cycle) {
        super::super::host::cycle(self.app, self.session, setting).await;
    }
}

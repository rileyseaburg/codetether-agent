//! Native setters and state binding for shared preference commands.
use crate::session::Session;
use crate::tui::app::{
    auto_apply::set_auto_apply_edits,
    settings::{set_network_access, set_slash_autocomplete},
    state::App,
};
use codetether_settings_policy::command::{Host, Setting};
struct Settings<'a> {
    app: &'a mut App,
    session: &'a mut Session,
}
impl Host for Settings<'_> {
    fn current(&self, setting: Setting) -> bool {
        match setting {
            Setting::AutoApply => self.app.state.auto_apply_edits,
            Setting::Network => self.app.state.allow_network,
            Setting::Autocomplete => self.app.state.slash_autocomplete,
        }
    }
    async fn apply(&mut self, setting: Setting, enabled: bool) {
        match setting {
            Setting::AutoApply => set_auto_apply_edits(self.app, self.session, enabled).await,
            Setting::Network => set_network_access(self.app, self.session, enabled).await,
            Setting::Autocomplete => set_slash_autocomplete(self.app, self.session, enabled).await,
        }
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
}
pub(super) async fn execute(app: &mut App, session: &mut Session, setting: Setting, rest: &str) {
    codetether_settings_policy::command::execute(&mut Settings { app, session }, setting, rest)
        .await;
}

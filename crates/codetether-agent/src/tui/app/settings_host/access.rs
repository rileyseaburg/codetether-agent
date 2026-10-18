//! Native trust, session and approval bindings for access-mode transitions.
use crate::{
    config::{AccessMode, Config},
    session::Session,
    tui::app::state::App,
};
struct Access<'a> {
    app: &'a mut App,
    session: &'a mut Session,
}
impl codetether_settings_policy::access_update::Host for Access<'_> {
    fn current_mode(&self) -> AccessMode {
        crate::tui::app::settings::access_mode::current_access_mode()
    }
    fn apply_config(&mut self, config: &Config) {
        crate::tui::ui::trust_status::set_from_config(config);
        self.session.apply_config(config, None);
    }
    fn release_all(&mut self) -> Result<usize, String> {
        crate::tui::app::input::approval_command::release_all(self.app)
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
}
pub(super) async fn cycle(app: &mut App, session: &mut Session) {
    codetether_settings_policy::access_update::cycle(&mut Access { app, session }).await;
}

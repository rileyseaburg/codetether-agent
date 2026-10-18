//! Native goal authorization and result-presentation bindings.
mod host;
use crate::tui::app::state::App;
impl codetether_goal_command::presentation::View for App {
    fn push_system_message(&mut self, message: String) {
        super::push_system_message(self, message);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
}
pub(crate) async fn handle(app: &mut App, session_id: &str, raw: &str) -> bool {
    codetether_goal_command::presentation::handle::<host::Native>(app, session_id, raw).await
}

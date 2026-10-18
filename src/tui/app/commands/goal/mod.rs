//! Native UI projection for explicit human goal commands.
mod host;
use crate::tui::app::state::App;
pub(crate) async fn handle(app: &mut App, session_id: &str, raw: &str) -> bool {
    let result = codetether_goal_command::run::<host::Native>(session_id, raw).await;
    match result {
        Ok(message) => {
            super::push_system_message(app, message.clone());
            app.state.status = message.lines().next().unwrap_or("Goal updated").to_string();
            true
        }
        Err(error) => {
            app.state.status = format!("/goal: {error}");
            false
        }
    }
}

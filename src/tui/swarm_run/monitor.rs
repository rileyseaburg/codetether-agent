//! Bare `/swarm` command: open the monitor or show launch guidance.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::ViewMode;

/// Handle the bare `/swarm` command.
///
/// If a swarm is already running, opens the live monitor view. Otherwise
/// shows usage guidance in chat instead of a confusing empty monitor — the
/// monitor opens automatically once `/swarm <task>` launches a run.
pub fn open_swarm_monitor(app: &mut App) {
    if app.state.swarm.active {
        app.state.set_view_mode(ViewMode::Swarm);
        app.state.status = "Swarm monitor".to_string();
        return;
    }
    app.state.status = "Usage: /swarm <task> to launch a parallel swarm".to_string();
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        "No swarm is running. To launch one, type:\n\
         /swarm <task description>\n\
         Example: /swarm research the auth module, then propose tests\n\
         The monitor opens automatically once a swarm starts."
            .to_string(),
    ));
    app.state.scroll_to_bottom();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_swarm_without_active_run_shows_guidance_and_stays_in_chat() {
        let mut app = App::default();
        app.state.set_view_mode(ViewMode::Chat);
        open_swarm_monitor(&mut app);
        assert_eq!(app.state.view_mode, ViewMode::Chat);
        assert!(app.state.status.contains("/swarm <task>"));
        let last = app.state.messages.last().expect("guidance message");
        assert!(matches!(last.message_type, MessageType::System));
    }

    #[test]
    fn bare_swarm_with_active_run_opens_monitor() {
        let mut app = App::default();
        app.state.swarm.active = true;
        open_swarm_monitor(&mut app);
        assert_eq!(app.state.view_mode, ViewMode::Swarm);
    }
}

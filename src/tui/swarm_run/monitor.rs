//! Application binding for swarm monitor guidance.
use crate::tui::app::state::App;
#[cfg(test)]
use crate::tui::{chat::message::MessageType, models::ViewMode};
/// Open the monitor or present swarm launch guidance.
pub fn open_swarm_monitor(app: &mut App) {
    codetether_swarm_view::interactive::open_swarm_monitor(app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_swarm_without_active_run_shows_guidance_and_stays_in_chat() {
        let mut app = App::default();
        crate::tui::app::state::view::transition(&mut app.state).set_view_mode(ViewMode::Chat);
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

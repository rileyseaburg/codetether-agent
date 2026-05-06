use crate::bus::BusMessage;
use crate::tui::app::state::App;

/// Update app state based on a bus message (agent presence tracking).
pub fn track(app: &mut App, message: &BusMessage) {
    match message {
        BusMessage::AgentReady { agent_id, .. } => {
            app.state.worker_bridge_registered_agents.insert(agent_id.clone());
        }
        BusMessage::AgentShutdown { agent_id } => {
            app.state.worker_bridge_registered_agents.remove(agent_id.as_str());
        }
        _ => {}
    }
}

use crate::bus::BusMessage;
use crate::tui::app::state::App;

/// Update app state based on a bus message (agent presence tracking).
///
/// Heartbeats are how A2A/mDNS-discovered peers announce themselves, so they
/// register too. `AgentShutdown` removes a peer (used by mDNS TTL expiry).
pub fn track(app: &mut App, message: &BusMessage) {
    app.state.active_tasks.observe(message);
    match message {
        BusMessage::AgentReady { agent_id, .. } | BusMessage::Heartbeat { agent_id, .. } => {
            app.state
                .worker_bridge_registered_agents
                .insert(agent_id.clone());
        }
        BusMessage::AgentShutdown { agent_id } => {
            app.state
                .worker_bridge_registered_agents
                .remove(agent_id.as_str());
        }
        _ => {}
    }
}

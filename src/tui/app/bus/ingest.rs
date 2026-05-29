use crate::bus::{BusHandle, BusMessage};
use crate::tui::app::state::App;

pub fn drain(app: &mut App, bus_handle: &mut BusHandle) {
    while let Some(envelope) = bus_handle.try_recv() {
        super::agent_track::track(app, &envelope.message);
        super::inbox::maybe_queue(app, &envelope);
        app.state.bus_log.ingest(&envelope);
    }
}

pub fn track_protocol_agent(app: &mut App, message: &BusMessage) {
    match message {
        BusMessage::AgentReady { agent_id, .. } | BusMessage::Heartbeat { agent_id, .. } => {
            app.state
                .worker_bridge_registered_agents
                .insert(agent_id.clone());
        }
        _ => {}
    }
}

use crate::a2a::types::Part;
use crate::bus::{BusEnvelope, BusHandle, BusMessage};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::IncomingTask;

pub fn drain(app: &mut App, bus_handle: &mut BusHandle) {
    while let Some(envelope) = bus_handle.try_recv() {
        crate::tui::app::bus_agent_track::track(app, &envelope.message);
        enqueue_agent_message(app, &envelope);
        app.state.bus_log.ingest(&envelope);
    }
}

fn enqueue_agent_message(app: &mut App, envelope: &BusEnvelope) {
    let BusMessage::AgentMessage { from, to, parts } = &envelope.message else {
        return;
    };
    if !envelope.topic.starts_with("agent.") {
        return;
    }
    if from == to || from == "a2a" || from == "a2a-stream" || !addressed_to_tui(app, to) {
        return;
    }
    let message = text_parts(parts);
    if !message.trim().is_empty() {
        app.state.enqueue_worker_task(IncomingTask {
            task_id: envelope
                .correlation_id
                .clone()
                .unwrap_or_else(|| envelope.id.clone()),
            message,
            from_agent: Some(from.clone()),
        });
    }
}

fn addressed_to_tui(app: &App, to: &str) -> bool {
    to == "tui"
        || app.state.worker_id.as_deref() == Some(to)
        || app.state.worker_bridge_registered_agents.contains(to)
}

fn text_parts(parts: &[Part]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            Part::Text { text } => Some(text.as_str()),
            Part::File { .. } | Part::Data { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

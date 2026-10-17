//! Inbound and outbound task messages on the application bus.
use super::*;

pub(super) fn emit_a2a_inbound(server: &A2AServer, task_id: &str, message: &Message) {
    emit_a2a_message(
        server,
        task_id,
        "remote-a2a",
        &server.agent_card.name,
        message,
    );
}

pub(super) fn emit_a2a_outbound(server: &A2AServer, task_id: &str, message: &Message) {
    emit_a2a_message(
        server,
        task_id,
        &server.agent_card.name,
        "remote-a2a",
        message,
    );
}

pub(super) fn emit_a2a_message(
    server: &A2AServer,
    task_id: &str,
    from: &str,
    to: &str,
    message: &Message,
) {
    let Some(bus) = server.bus.as_ref() else {
        return;
    };
    let handle = bus.handle("a2a");
    handle.send_with_correlation(
        format!("task.{task_id}"),
        BusMessage::AgentMessage {
            from: from.to_string(),
            to: to.to_string(),
            parts: message.parts.clone(),
        },
        Some(task_id.to_string()),
    );
}

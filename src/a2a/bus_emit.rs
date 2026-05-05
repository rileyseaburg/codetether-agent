use std::sync::Arc;

use crate::a2a::types::Message;
use crate::bus::{AgentBus, BusMessage};

pub fn emit(bus: &Arc<AgentBus>, task_id: &str, from: &str, to: &str, message: &Message) {
    let payload = BusMessage::AgentMessage {
        from: from.to_string(),
        to: to.to_string(),
        parts: message.parts.clone(),
    };
    let handle = bus.handle("a2a");
    handle.send_with_correlation(
        format!("task.{task_id}"),
        payload.clone(),
        Some(task_id.into()),
    );
    if from == "remote-a2a" {
        handle.send_with_correlation(format!("a2a.{to}"), payload, Some(task_id.into()));
    }
}

//! Drain a bounded number of bus observations to the forage console.
use super::*;
use crate::forage_println;

pub(super) fn flush_bus_observer(observer: &mut BusHandle, cycle: usize, json_mode: bool) {
    if json_mode {
        return;
    }

    let mut shown = 0usize;
    while let Some(env) = observer.try_recv() {
        if shown >= 12 {
            break;
        }
        let label = match &env.message {
            BusMessage::AgentReady { .. } => "agent_ready",
            BusMessage::AgentShutdown { .. } => "agent_shutdown",
            BusMessage::TaskUpdate { .. } => "task_update",
            BusMessage::SharedResult { .. } => "shared_result",
            BusMessage::AgentMessage { .. } => "agent_message",
            _ => "other",
        };
        forage_println!(
            "   [bus cycle {cycle}] {} :: topic={} sender={}",
            label,
            env.topic,
            env.sender_id
        );
        shown = shown.saturating_add(1);
    }
}

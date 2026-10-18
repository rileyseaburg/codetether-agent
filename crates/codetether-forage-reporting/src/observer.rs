//! Drain a bounded number of bus observations to the forage console.
use super::*;
use crate::forage_println;

/// Drain and display bounded bus observations for a human-readable cycle.
/// # Arguments
/// * `observer` — Bus subscription whose queued events are consumed.
/// * `cycle` — Cycle number used in display labels.
/// * `json_mode` — Return immediately without draining when true.
/// # Returns
/// Displays at most twelve events using the shared quiet-mode gate.
/// # Examples
/// ```no_run
/// # fn display(observer: &mut codetether_bus_core::BusHandle) {
/// codetether_forage_reporting::flush_bus_observer(observer, 1, false);
/// # }
/// ```
pub fn flush_bus_observer(observer: &mut BusHandle, cycle: usize, json_mode: bool) {
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

use crate::{WorkerBridgeCmd, WorkerState};
use std::collections::HashSet;
use tokio::sync::mpsc;

/// Reconcile bridge registrations with agents observed in READY messages.
///
/// # Arguments
/// * `state` - Local registration cache updated after the send attempts.
/// * `sender` - Optional bridge command sender; absence leaves state untouched.
/// * `messages` - Borrowed `(kind, summary)` pairs from the observed bus log.
/// # Returns
/// Reports registration differences without waiting and stores the desired set.
/// # Examples
/// ```
/// use codetether_tui_worker::{synchronization, WorkerState};
/// let mut state = WorkerState::default();
/// synchronization::sync_agents(&mut state, None, [("READY", "helper online")]);
/// assert!(state.worker_bridge_registered_agents.is_empty());
/// ```
pub fn sync_agents<'a>(
    state: &mut WorkerState,
    sender: Option<&mpsc::Sender<WorkerBridgeCmd>>,
    messages: impl IntoIterator<Item = (&'a str, &'a str)>,
) {
    let Some(sender) = sender else {
        return;
    };
    let mut desired = HashSet::new();
    desired.insert("tui".to_string());
    for (kind, summary) in messages {
        if kind == "READY"
            && let Some(agent) = summary.split_whitespace().next()
        {
            desired.insert(agent.to_string());
        }
    }
    let current = state.worker_bridge_registered_agents.clone();
    for name in desired.difference(&current) {
        let _ = sender.try_send(WorkerBridgeCmd::RegisterAgent {
            name: name.clone(),
            instructions: format!("Protocol-observed A2A agent {name}"),
        });
    }
    for name in current.difference(&desired) {
        let _ = sender.try_send(WorkerBridgeCmd::DeregisterAgent { name: name.clone() });
    }
    state.worker_bridge_registered_agents = desired;
}

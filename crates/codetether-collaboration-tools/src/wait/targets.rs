//! Resolve requested targets into status subscriptions.

use crate::{ThreadStatus, WaitHost};
use anyhow::Result;
use serde_json::{Map, Value, to_value};
use tokio::sync::watch;

pub(super) type Receiver = watch::Receiver<ThreadStatus>;

pub(super) fn resolve<H: WaitHost>(
    owner: &str,
    targets: &[String],
) -> Result<(Map<String, Value>, Vec<(String, Receiver)>)> {
    let mut final_status = Map::new();
    let mut receivers = Vec::new();
    for target in targets {
        let Some(agent) = H::find_agent(target, owner) else {
            final_status.insert(target.clone(), to_value(ThreadStatus::NotFound)?);
            continue;
        };
        ensure_status::<H>(&agent.id, agent.is_processing);
        let receiver = H::subscribe(&agent.id).expect("status initialized");
        let current = receiver.borrow().clone();
        if current.is_final() {
            final_status.insert(agent.id, to_value(current)?);
        } else {
            receivers.push((agent.id, receiver));
        }
    }
    Ok((final_status, receivers))
}

fn ensure_status<H: WaitHost>(agent_id: &str, running: bool) {
    if H::subscribe(agent_id).is_some() {
        return;
    }
    let status = if running {
        ThreadStatus::Running
    } else {
        ThreadStatus::Completed(None)
    };
    H::set_status(agent_id, status);
}

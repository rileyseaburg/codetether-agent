//! Resolve requested targets into status subscriptions.

use crate::tool::agent::collaboration_runtime::thread_status::{self, ThreadStatus};
use anyhow::Result;
use serde_json::{Map, Value, to_value};
use tokio::sync::watch;

pub(super) type Receiver = watch::Receiver<ThreadStatus>;

pub(super) fn resolve(
    owner: &str,
    targets: &[String],
) -> Result<(Map<String, Value>, Vec<(String, Receiver)>)> {
    let mut final_status = Map::new();
    let mut receivers = Vec::new();
    for target in targets {
        let Some(agent) = crate::tool::agent::bridge::find_agent_tool_agent_for_parent(target, owner)
        else {
            final_status.insert(target.clone(), to_value(ThreadStatus::NotFound)?);
            continue;
        };
        ensure_status(&agent.id, agent.is_processing);
        let receiver = thread_status::subscribe(&agent.id).expect("status initialized");
        let current = receiver.borrow().clone();
        if current.is_final() {
            final_status.insert(agent.id, to_value(current)?);
        } else {
            receivers.push((agent.id, receiver));
        }
    }
    Ok((final_status, receivers))
}

fn ensure_status(agent_id: &str, running: bool) {
    if thread_status::subscribe(agent_id).is_some() {
        return;
    }
    let status = if running {
        ThreadStatus::Running
    } else {
        ThreadStatus::Completed(None)
    };
    thread_status::set(agent_id, status);
}

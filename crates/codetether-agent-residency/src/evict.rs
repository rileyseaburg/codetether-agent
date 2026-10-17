//! Safe unloading of one least-recently-used resident child.

use crate::Host;
use codetether_child_mailbox as message_queue;
use codetether_collaboration_state::execution as execution_state;
use codetether_collaboration_state::{residency as state, thread_status};
use thread_status::ThreadStatus;

pub(super) async fn one<H: Host>(protected: Option<&str>) -> bool {
    let count = state::resident_count();
    for _ in 0..count {
        let Some(agent_id) = state::pop_candidate(protected) else {
            return false;
        };
        let Some(entry) = H::get(&agent_id) else {
            continue;
        };
        if !unloadable(&agent_id).await {
            state::touch(&agent_id);
            continue;
        }
        if let Err(error) = H::close(&entry.name, &entry).await {
            tracing::warn!(target: "codetether_agent::tool::agent::residency::evict", agent_id, %error, "Failed to unload resident child");
            state::touch(&agent_id);
            continue;
        }
        execution_state::close(&agent_id);
        thread_status::shutdown(&agent_id);
        H::remove_runtime(&agent_id);
        tracing::info!(target: "codetether_agent::tool::agent::residency::evict", agent_id, agent = %entry.name, "Unloaded LRU child for residency capacity");
        return true;
    }
    false
}

async fn unloadable(agent_id: &str) -> bool {
    if execution_state::is_running(agent_id) || message_queue::pending(agent_id).await > 0 {
        return false;
    }
    matches!(
        thread_status::get(agent_id),
        ThreadStatus::Completed(_) | ThreadStatus::Errored(_) | ThreadStatus::Interrupted
    )
}

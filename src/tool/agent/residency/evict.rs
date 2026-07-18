//! Safe unloading of one least-recently-used resident child.

use super::super::collaboration_runtime::{message_queue, thread_status};
use super::super::{execution_state, persistence, store};
use thread_status::ThreadStatus;

pub(super) async fn one(protected: Option<&str>) -> bool {
    let count = super::order::resident_count();
    for _ in 0..count {
        let Some(agent_id) = super::order::pop_candidate(protected) else {
            return false;
        };
        let Some(entry) = store::get(&agent_id) else {
            continue;
        };
        if !unloadable(&agent_id).await {
            super::order::touch(&agent_id);
            continue;
        }
        if let Err(error) = persistence::close(&entry.name, &entry).await {
            tracing::warn!(agent_id, %error, "Failed to unload resident child");
            super::order::touch(&agent_id);
            continue;
        }
        execution_state::close(&agent_id);
        thread_status::shutdown(&agent_id);
        store::remove(&agent_id);
        super::super::event_loop::live_trace::clear(&agent_id);
        tracing::info!(agent_id, agent = %entry.name, "Unloaded LRU child for residency capacity");
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

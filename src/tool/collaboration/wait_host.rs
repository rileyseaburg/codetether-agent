//! Bind waits to the existing child registry, statuses and parent notifications.
use super::host::ApplicationHost;
use crate::tool::agent::{
    bridge,
    collaboration_runtime::{parent_activity, thread_status},
};
use codetether_collaboration_tools::{Activity, ThreadStatus, WaitAgent};
use tokio::{sync::watch, time::Instant};
#[async_trait::async_trait]
impl codetether_collaboration_tools::WaitHost for ApplicationHost {
    fn find_agent(target: &str, owner: &str) -> Option<WaitAgent> {
        bridge::find_agent_tool_agent_for_parent(target, owner).map(|agent| WaitAgent {
            id: agent.id,
            is_processing: agent.is_processing,
        })
    }
    fn subscribe(agent_id: &str) -> Option<watch::Receiver<ThreadStatus>> {
        thread_status::subscribe(agent_id)
    }
    fn set_status(agent_id: &str, status: ThreadStatus) {
        thread_status::set(agent_id, status)
    }
    async fn activity(owner: &str, deadline: Instant) -> Option<Activity> {
        parent_activity::until(owner, deadline).await
    }
}

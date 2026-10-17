//! Bind shared child-status reporting to live registry snapshots.
use super::{bus_publish::lifecycle, store};
use crate::tool::ToolResult;
use codetether_agent_status::{AgentView, Host};
use std::collections::HashMap;
struct ApplicationHost;
impl Host for ApplicationHost {
    fn agents(parent: Option<&str>) -> Vec<AgentView> {
        store::entries_for_parent(parent)
            .into_iter()
            .map(|entry| AgentView {
                id: entry.id().to_string(),
                messages: entry.session.messages.len(),
                name: entry.name,
            })
            .collect()
    }
    fn task_to_agent(parent: Option<&str>) -> HashMap<String, String> {
        lifecycle::task_to_agent(parent)
    }
}
pub(super) fn handle_status(parent: Option<&str>) -> ToolResult {
    codetether_agent_status::report::<ApplicationHost>(parent)
}

//! Shared lifecycle summaries and owned task identities.
pub(in crate::tool::agent) use codetether_agent_status::terminal_summary;
use std::collections::HashMap;
pub(in crate::tool::agent) fn task_to_agent(parent: Option<&str>) -> HashMap<String, String> {
    codetether_agent_status::task_to_agent::<crate::tool::agent::status::ApplicationHost>(parent)
}

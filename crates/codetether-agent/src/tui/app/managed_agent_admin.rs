//! Administrative commands for managed child agents.

#[path = "managed_agent_kill.rs"]
mod remove;
#[path = "managed_agent_list.rs"]
mod show;

pub(crate) use remove::kill;
pub(crate) use show::list;

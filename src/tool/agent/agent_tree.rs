//! Canonical live-agent tree queries for MultiAgentV2 tools.

#[path = "agent_tree/path.rs"]
mod path;
#[path = "agent_tree/prefix.rs"]
mod prefix;
#[path = "agent_tree/query.rs"]
mod query;
#[path = "agent_tree/status.rs"]
mod status;
#[path = "agent_tree/target.rs"]
mod target;

pub(crate) use query::list;
pub(crate) use target::{canonical, resolve};

#[cfg(test)]
#[path = "agent_tree/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "agent_tree/tests.rs"]
mod tests;

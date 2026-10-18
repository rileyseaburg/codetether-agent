//! Application snapshots for shared root-scoped agent-tree queries.
#[path = "agent_tree/host.rs"]
mod host;
use codetether_agent_tree as query;
pub(crate) use query::ListedAgent;
pub(crate) fn list(current: &str, path_prefix: Option<&str>) -> anyhow::Result<Vec<ListedAgent>> {
    codetether_agent_tree::list::<host::ApplicationHost>(current, path_prefix)
}
pub(crate) fn resolve(current: &str, target: &str) -> anyhow::Result<Option<String>> {
    codetether_agent_tree::resolve::<host::ApplicationHost>(current, target)
}
pub(crate) fn canonical(current: &str, target: &str) -> anyhow::Result<Option<String>> {
    codetether_agent_tree::canonical::<host::ApplicationHost>(current, target)
}
#[cfg(test)]
#[path = "agent_tree/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "agent_tree/tests.rs"]
mod tests;

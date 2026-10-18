//! Shared durable child lookup and test-only lifecycle inspection.
#[cfg(test)]
use anyhow::Result;
pub(in crate::tool::agent) use codetether_agent_storage::{child_ids, durable_id};
/// Return whether a test fixture's durable spawn edge is open.
#[cfg(test)]
pub(in crate::tool::agent) async fn is_open(owner: Option<&str>, target: &str) -> Result<bool> {
    Ok(super::manifest_scan::find(owner, target)
        .await?
        .is_some_and(|manifest| manifest.lifecycle == super::manifest::Lifecycle::Open))
}

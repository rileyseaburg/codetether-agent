//! Application binding for shared resident lookup and durable activation.
use super::{EnsureOpen, ResumeConfig, host::ApplicationHost};
pub(in crate::tool) async fn open(
    target: &str,
    owner: Option<&str>,
    config: ResumeConfig,
) -> anyhow::Result<EnsureOpen> {
    codetether_agent_residency::open::<ApplicationHost>(target, owner, config).await
}

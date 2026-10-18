//! Resolve and retain the remote provider registry and capability-scoped tools.
use super::RemoteHost;
use anyhow::{Result, anyhow};
use codetether_swarm_core::remote_protocol::RemoteSubtaskPayload;
use codetether_swarm_orchestrator::Host as Providers;
use codetether_swarm_remote_runtime::Resources;
use std::{path::PathBuf, sync::Arc};
pub(super) async fn resolve<H: RemoteHost>(
    payload: &RemoteSubtaskPayload,
    working_dir: &PathBuf,
) -> Result<Resources<<H::Providers as Providers>::Providers, H::Registry>> {
    let registry = H::Providers::load().await?;
    let provider = H::Providers::get(&registry, &payload.provider)
        .ok_or_else(|| anyhow!("Provider '{}' unavailable in remote pod", payload.provider))?;

    let capability =
        codetether_workspace_tools::Capability::from_flags(payload.read_only, payload.verification);
    let tool_registry = Arc::new(codetether_workspace_tools::composition::with_provider::<
        H::Workspace,
    >(
        Arc::clone(&provider),
        payload.model.clone(),
        working_dir,
        capability,
    ));
    let tool_defs = <H::Workspace as codetether_workspace_tools::composition::Factory>::definitions(
        &tool_registry,
    );

    Ok(Resources {
        _registry: registry,
        provider,
        tool_registry,
        tool_defs,
    })
}

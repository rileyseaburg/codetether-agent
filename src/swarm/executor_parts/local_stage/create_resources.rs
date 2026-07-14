//! Provider and worktree resources for a local stage.

use super::super::SwarmExecutor;
use crate::provider::{Provider, ProviderRegistry};
use crate::swarm::Orchestrator;
use crate::worktree::WorktreeManager;
use anyhow::{Result, anyhow};
use std::{path::Path, sync::Arc};

pub(super) fn provider<'a>(
    orchestrator: &'a Orchestrator,
    name: &str,
) -> Result<(&'a ProviderRegistry, Arc<dyn Provider>)> {
    let providers = orchestrator.providers();
    let fallback = providers
        .get(name)
        .ok_or_else(|| anyhow!("Provider {name} not found"))?;
    Ok((providers, fallback))
}

pub(super) fn manager(executor: &SwarmExecutor, workspace: &Path) -> Option<Arc<WorktreeManager>> {
    executor.config.worktree_enabled.then(|| {
        Arc::new(
            WorktreeManager::for_repo(workspace.display().to_string()).without_vscode_auto_open(),
        )
    })
}

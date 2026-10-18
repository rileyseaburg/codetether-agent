//! Provider and worktree resources for a local stage.

use crate::Host;
use anyhow::{Result, anyhow};
use codetether_provider_core::Provider;
use codetether_worktree::WorktreeManager;
use std::{path::Path, sync::Arc};

pub(super) fn provider<'a, H: Host>(
    providers: &'a H::Providers,
    name: &str,
) -> Result<(&'a H::Providers, Arc<dyn Provider>)> {
    let fallback = H::get(providers, name).ok_or_else(|| anyhow!("Provider {name} not found"))?;
    Ok((providers, fallback))
}

pub(super) fn manager(executor: &impl Host, workspace: &Path) -> Option<Arc<WorktreeManager>> {
    executor.config().worktree_enabled.then(|| {
        Arc::new(
            WorktreeManager::for_repo(workspace.display().to_string()).without_vscode_auto_open(),
        )
    })
}

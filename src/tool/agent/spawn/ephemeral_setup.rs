//! Provider, tools, prompt, and workspace for an ephemeral agent run.

use super::super::{helpers, spawn_request::SpawnRequest};
use crate::provider::Provider;
use crate::tool::ToolRegistry;
use anyhow::Result;
use std::{path::PathBuf, sync::Arc};

#[path = "ephemeral_task.rs"]
mod task;

pub(super) struct Setup {
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) prompt: String,
    pub(super) registry: Arc<ToolRegistry>,
    pub(super) workspace: PathBuf,
}

pub(super) async fn prepare(request: &SpawnRequest<'_>) -> Result<Setup> {
    let providers = helpers::get_registry().await?;
    let (provider, model) = providers.resolve_model(request.model)?;
    let (workspace, read_only, expects_changes) = task::policy(request);
    let registry = crate::tool::swarm_execute::agent_registry::standard(
        read_only,
        !read_only && !expects_changes,
        &workspace,
        Arc::clone(&provider),
        model.clone(),
    );
    let prompt = crate::tool::swarm_execute::agent_prompt::build(
        request.name,
        None,
        &workspace,
        &model,
        request.instructions,
        read_only,
        expects_changes,
    );
    Ok(Setup {
        provider,
        model,
        prompt,
        registry,
        workspace,
    })
}

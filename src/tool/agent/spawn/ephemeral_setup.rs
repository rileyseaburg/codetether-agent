//! Provider, tools, prompt, and workspace for an ephemeral agent run.

use super::super::{helpers, spawn_request::SpawnRequest};
use anyhow::Result;
use std::sync::Arc;

#[path = "ephemeral_context.rs"]
mod context;
pub(super) use context::Setup;

#[path = "ephemeral_task.rs"]
mod task;

pub(super) async fn prepare(request: &SpawnRequest<'_>) -> Result<Setup> {
    let providers = helpers::get_registry().await?;
    let (provider, model) = providers.resolve_model(request.model)?;
    let (workspace, read_only, expects_changes, handoff) = task::policy(request).await?;
    let registry = crate::tool::swarm_execute::agent_registry::standard(
        read_only,
        !read_only && !expects_changes,
        &workspace,
        Arc::clone(&provider),
        model.clone(),
    );
    let mut prompt = crate::tool::swarm_execute::agent_prompt::build(
        request.name,
        None,
        &workspace,
        &model,
        request.instructions,
        read_only,
        expects_changes,
    );
    if let Some(handoff) = &handoff {
        prompt.push_str(&format!("\n\n{}", handoff.guidance()));
    }
    Ok(Setup {
        provider,
        model,
        prompt,
        registry,
        workspace,
        handoff,
    })
}

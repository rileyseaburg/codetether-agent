//! Provider, tools, prompt, and workspace for an ephemeral agent run.

use crate::{EphemeralHost, EphemeralRun};
use anyhow::Result;
use codetether_agent_input::SpawnRequest;
use std::sync::Arc;

#[path = "ephemeral_context.rs"]
mod context;
use context::Setup;

#[path = "ephemeral_task.rs"]
pub(crate) mod task;

pub(super) async fn prepare<H: EphemeralHost>(
    request: &SpawnRequest<'_>,
) -> Result<Setup<H::Registry>> {
    let (provider, model) = H::resolve(request.model).await?;
    let (workspace, read_only, expects_changes, handoff) =
        task::workspace_policy::<H>(request).await?;
    let registry = H::registry(
        read_only,
        !read_only && !expects_changes,
        &workspace,
        Arc::clone(&provider),
        model.clone(),
    );
    let mut prompt = H::prompt(request, &workspace, &model, read_only, expects_changes);
    if let Some(handoff) = &handoff {
        prompt.push_str(&format!("\n\n{}", handoff.guidance()));
    }
    Ok(Setup {
        run: EphemeralRun {
            provider,
            model,
            prompt,
            registry,
            workspace,
        },
        handoff,
    })
}

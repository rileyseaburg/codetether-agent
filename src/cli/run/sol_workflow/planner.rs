//! One-shot, tool-free completions from the Sol planning model.

use anyhow::Result;

use crate::provider::ProviderRegistry;
use codetether_sol_workflow::{SOL_PLANNER_MODEL, request, text};

/// Ask Sol for a plan or diagnostic review without advertising any tools.
pub(super) async fn complete(system: String, user: String) -> Result<String> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let (provider, model) = registry.resolve_model(SOL_PLANNER_MODEL)?;
    let response = provider.complete(request(model, system, user)).await?;
    Ok(text(&response.message.content))
}

//! One-shot, tool-free completions from the Sol planning model.
use crate::{SOL_PLANNER_MODEL, request, text};
use codetether_application_providers::{Application, ProviderRegistry};
/// Ask Sol for a plan or diagnostic review without advertising tools.
/// # Arguments
/// `system` and `user` supply the planning or review instructions.
/// # Returns
/// Text content extracted from the provider's response.
/// # Errors
/// Propagates registry, model resolution and completion errors.
/// # Examples
/// ```no_run
/// # async fn example<A: codetether_application_providers::Application>() -> anyhow::Result<()> {
/// let _plan = codetether_sol_workflow::planner::complete::<A>("Plan only".into(), "Inspect the task".into()).await?;
/// # Ok(()) }
/// ```
pub async fn complete<A: Application>(system: String, user: String) -> anyhow::Result<String> {
    let registry = ProviderRegistry::<A>::shared_from_vault().await?;
    let (provider, model) = registry.resolve_model(SOL_PLANNER_MODEL)?;
    let response = provider.complete(request(model, system, user)).await?;
    Ok(text(&response.message.content))
}

//! Ordered depth admission, provider loading and advisory model-cost policy.
use crate::ModelRegistry;
use codetether_tool_core::ToolResult;
use std::{future::Future, sync::Arc};

/// Validate child depth before loading providers and checking model eligibility.
/// # Arguments
/// Receives the child name, parent-derived depth, requested model and a future
/// loading the application's existing provider registry.
/// # Returns
/// An advisory cost warning for an ineligible model, or `None` when eligible.
/// # Errors
/// Returns a tool error for invalid depth, configuration or registry loading.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_spawn_policy::ModelRegistry>(registry: std::sync::Arc<R>) {
/// let outcome = codetether_spawn_policy::admission::validate(
///     "reviewer", 1, "provider/model", async { Ok(registry) },
/// ).await;
/// # }
/// ```
pub async fn validate<R: ModelRegistry>(
    name: &str,
    depth: u8,
    model: &str,
    registry: impl Future<Output = anyhow::Result<Arc<R>>>,
) -> Result<Option<String>, ToolResult> {
    crate::validate_depth(name, depth).await?;
    let registry = match registry.await {
        Ok(registry) => registry,
        Err(error) => return Err(ToolResult::error(error.to_string())),
    };
    Ok(crate::model_warning(model, registry.as_ref()).await)
}

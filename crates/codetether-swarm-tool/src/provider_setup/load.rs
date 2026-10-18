//! Provider loading and model resolution for direct swarm execution.

use super::Registry;
use crate::runtime::SelectedProvider;
use crate::{model_selection, runtime::Observer};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use std::{future::Future, sync::Arc};

/// Load provider availability and resolve the requested worker model.
/// # Arguments
/// * `requested` — Explicit or inherited model reference.
/// * `observer` — Invocation observer receiving terminal setup errors.
/// * `loader` — Native registry-loading future, awaited once.
/// # Returns
/// A selected provider, or an inner tool error for unavailable providers/models.
/// # Errors
/// The outer error preserves registry-loading failures with setup context.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_swarm_tool::provider_setup::Registry, O: codetether_swarm_tool::runtime::Observer>(registry: std::sync::Arc<R>, observer: &mut O) -> anyhow::Result<()> {
/// let selected = codetether_swarm_tool::provider_setup::load(None, observer, std::future::ready(Ok(registry))).await?;
/// # let _ = selected;
/// # Ok(()) }
/// ```
pub async fn load<R: Registry>(
    requested: Option<&str>,
    observer: &mut impl Observer,
    loader: impl Future<Output = Result<Arc<R>>>,
) -> Result<Result<SelectedProvider, ToolResult>> {
    let providers = match loader.await {
        Ok(providers) => providers,
        Err(error) => {
            observer.fail(format!("Failed to load providers: {error:#}"));
            return Err(error).context("Failed to load providers");
        }
    };
    let available = providers.list();
    if available.is_empty() {
        return Ok(Err(fail(
            observer,
            "No providers available for swarm execution",
        )));
    }
    let model = match model_selection::resolve(requested, &available) {
        Ok(model) => model,
        Err(error) => return Ok(Err(fail(observer, &error.to_string()))),
    };
    let Some(provider) = providers.get(&model.resolved_provider) else {
        let error = format!(
            "Resolved provider '{}' disappeared before execution",
            model.resolved_provider
        );
        return Ok(Err(fail(observer, &error)));
    };
    Ok(Ok(SelectedProvider { provider, model }))
}

fn fail(observer: &mut impl Observer, error: &str) -> ToolResult {
    observer.fail(error);
    ToolResult::error(error)
}

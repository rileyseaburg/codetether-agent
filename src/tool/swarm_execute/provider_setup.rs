//! Provider loading and model resolution for direct swarm execution.

use super::{model_selection, tui_bridge::Observer};
use crate::provider::{Provider, ProviderRegistry};
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use std::sync::Arc;

pub(super) struct SelectedProvider {
    pub provider: Arc<dyn Provider>,
    pub model: model_selection::ModelSelection,
}

pub(super) async fn load(
    requested: Option<&str>,
    observer: &mut Observer,
) -> Result<Result<SelectedProvider, ToolResult>> {
    let providers = match ProviderRegistry::from_vault().await {
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

fn fail(observer: &mut Observer, error: &str) -> ToolResult {
    observer.fail(error);
    ToolResult::error(error)
}

//! Concurrent provider discovery with bounded per-provider wait.
use crate::{ModelRefreshSummary, Registry};
use futures::future::join_all;
use std::time::Duration;

pub(crate) const MODEL_LIST_TIMEOUT: Duration = Duration::from_secs(4);

/// Discover selectable model identifiers from configured providers.
/// # Arguments
/// * `registry` - Host provider lookup, preserving provider instances.
/// # Returns
/// Sorted, deduplicated identifiers and counts. Individual failures and
/// four-second timeouts are counted while successful providers still contribute.
/// # Examples
/// ```no_run
/// # async fn example(registry: &impl codetether_model_discovery::Registry) {
/// let (models, summary) = codetether_model_discovery::load_available_models(registry).await;
/// assert_eq!(models.len(), summary.loaded_models);
/// # }
/// ```
pub async fn load_available_models(
    registry: &(impl Registry + ?Sized),
) -> (Vec<String>, ModelRefreshSummary) {
    let provider_names: Vec<String> = registry.list().into_iter().map(str::to_string).collect();
    let provider_count = provider_names.len();

    let fetches = provider_names.into_iter().filter_map(|provider_name| {
        registry.get(&provider_name).map(|provider| async move {
            let result = tokio::time::timeout(MODEL_LIST_TIMEOUT, provider.list_models()).await;
            (provider_name, result)
        })
    });

    let mut models = Vec::new();
    let mut summary = ModelRefreshSummary {
        provider_count,
        ..ModelRefreshSummary::empty()
    };

    for (provider_name, result) in join_all(fetches).await {
        super::outcome::apply(&mut models, &mut summary, &provider_name, result);
    }

    models.sort();
    models.dedup();
    summary.loaded_models = models.len();
    (models, summary)
}

//! Provider model collection for model discovery.

use super::{convert, types};
use futures::future::join_all;

pub(crate) async fn collect_models(
    registry: &crate::provider::ProviderRegistry,
) -> Vec<types::Model> {
    let futures = registry.list().into_iter().map(|provider_id| async move {
        let Some(provider) = registry.get(provider_id) else {
            return Vec::new();
        };
        match provider.list_models().await {
            Ok(models) => models
                .into_iter()
                .map(|model| convert::convert_model(provider_id, model))
                .collect(),
            Err(error) => failed_provider(provider_id, error),
        }
    });
    let mut data: Vec<_> = join_all(futures).await.into_iter().flatten().collect();
    data.sort_by(|a: &types::Model, b| a.id.cmp(&b.id));
    data
}

fn failed_provider(provider_id: &str, error: anyhow::Error) -> Vec<types::Model> {
    tracing::warn!(provider = %provider_id, error = %error, "Failed to list models");
    Vec::new()
}

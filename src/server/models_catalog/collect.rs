//! Provider model collection for model discovery.

use super::{convert, types};
use futures::future::join_all;
use std::time::Duration;

const PROVIDER_TIMEOUT: Duration = Duration::from_secs(5);

pub(crate) async fn collect_models(
    registry: &crate::provider::ProviderRegistry,
    created: i64,
) -> Vec<types::Model> {
    let futures = registry.list().into_iter().map(|provider_id| async move {
        let Some(provider) = registry.get(provider_id) else {
            return Vec::new();
        };
        match tokio::time::timeout(PROVIDER_TIMEOUT, provider.list_models()).await {
            Ok(Ok(models)) => models
                .into_iter()
                .map(|model| convert::convert_model(provider_id, model, created))
                .collect(),
            Ok(Err(error)) => failed_provider(provider_id, error),
            Err(_) => failed_provider(provider_id, anyhow::anyhow!("Timeout after 5s")),
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

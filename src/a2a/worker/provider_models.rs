//! Provider model loading helpers for worker registration.

use std::collections::HashMap;

use anyhow::Result;
use tokio::sync::OnceCell;

use crate::provider::{ModelInfo, ProviderRegistry};

pub(super) async fn load_provider_models() -> Result<HashMap<String, Vec<ModelInfo>>> {
    static CACHE: OnceCell<HashMap<String, Vec<ModelInfo>>> = OnceCell::const_new();
    CACHE
        .get_or_try_init(load_provider_models_uncached)
        .await
        .cloned()
}

async fn load_provider_models_uncached() -> Result<HashMap<String, Vec<ModelInfo>>> {
    let registry = match ProviderRegistry::shared_from_vault().await {
        Ok(registry) if !registry.list().is_empty() => (*registry).clone(),
        Ok(_) | Err(_) => fallback_registry().await?,
    };
    let mut models_by_provider = HashMap::new();
    for provider_name in registry.list() {
        if let Some(provider) = registry.get(provider_name)
            && let Ok(models) = provider.list_models().await
            && !models.is_empty()
        {
            models_by_provider.insert(provider_name.to_string(), models);
        }
    }
    Ok(models_by_provider)
}

async fn fallback_registry() -> Result<ProviderRegistry> {
    let config = crate::config::Config::load().await.unwrap_or_default();
    ProviderRegistry::from_config(&config).await
}

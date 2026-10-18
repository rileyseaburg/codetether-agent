//! Provider model loading helpers for worker registration.

use std::collections::HashMap;

use crate::ModelInfo;
use anyhow::Result;
use codetether_model_discovery::worker::Catalog;

/// Load advertised provider models through the shared process cache.
pub async fn load_provider_models() -> Result<HashMap<String, Vec<ModelInfo>>> {
    static CACHE: Catalog = Catalog::new();
    CACHE.load::<crate::provider::ProviderRegistry>().await
}

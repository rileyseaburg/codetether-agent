//! Provider model loading helpers for worker registration.

use std::collections::HashMap;

use crate::provider::ModelInfo;
use anyhow::Result;
use codetether_model_discovery::worker::Catalog;
mod host;

pub(super) async fn load_provider_models() -> Result<HashMap<String, Vec<ModelInfo>>> {
    static CACHE: Catalog = Catalog::new();
    CACHE.load::<host::Native>().await
}

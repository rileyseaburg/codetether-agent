//! Serve model discovery from the application's shared provider registry.
use crate::{Application, ProviderRegistry};
use std::sync::Arc;

impl<H: Application> codetether_model_catalog::http::Host for ProviderRegistry<H> {
    type Registry = Self;
    async fn load_registry() -> anyhow::Result<Arc<Self>> {
        Self::shared_from_vault().await
    }
}

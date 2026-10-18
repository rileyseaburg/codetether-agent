//! session_model provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use anyhow::Result;
use std::sync::Arc;
impl<H: Application> codetether_session_model_policy::ModelResolver for ProviderRegistry<H> {
    fn resolve_model(&self, selector: &str) -> Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, selector)
    }
}

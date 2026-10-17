//! Application model discovery delegation.
use super::ZaiProvider;
use crate::ModelInfo;
impl ZaiProvider {
    pub(super) async fn discover_models_from_api(&self) -> Vec<ModelInfo> {
        crate::discovery::discover_application(&self.client, &self.api_key, &self.base_url).await
    }
}

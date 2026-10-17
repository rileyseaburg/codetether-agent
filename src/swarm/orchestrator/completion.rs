use super::*;
use async_trait::async_trait;
use codetether_swarm_core::planning::PlanCompletion;
pub(super) struct ProviderCompletion<'a> {
    pub(super) provider: &'a dyn crate::provider::Provider,
    pub(super) model: &'a str,
    pub(super) temperature: f32,
}
#[async_trait]
impl PlanCompletion for ProviderCompletion<'_> {
    async fn complete(&self, prompt: String) -> Result<String> {
        super::decomposition_request::complete(self.provider, self.model, prompt, self.temperature)
            .await
    }
}

pub(super) fn temperature(model: &str) -> f32 {
    if crate::session::helper::provider::prefers_temperature_one(model) {
        1.0
    } else {
        0.7
    }
}

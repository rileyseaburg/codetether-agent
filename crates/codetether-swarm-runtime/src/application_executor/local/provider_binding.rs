//! Native provider binding for local stages.
use super::{LocalStage, LocalStageHost, providers};
use std::sync::Arc;
impl<H: LocalStageHost> codetether_swarm_local_stage::Providers for LocalStage<'_, H> {
    type Providers = H::Providers;

    fn get(
        providers: &Self::Providers,
        name: &str,
    ) -> Option<Arc<dyn codetether_provider_core::Provider>> {
        H::get(providers, name)
    }

    fn choose(
        &self,
        providers: &Self::Providers,
        specialty: &str,
        provider: &str,
        model: &str,
    ) -> (String, String) {
        providers::choose(self.0, providers, specialty, provider, model)
    }

    fn outcome(&self, provider: &str, specialty: &str, success: bool) {
        providers::outcome(self.0, provider, specialty, success);
    }
}

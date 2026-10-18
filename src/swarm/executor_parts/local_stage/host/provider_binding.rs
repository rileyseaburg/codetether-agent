//! Native provider binding for local stages.
use super::{Native, providers};
use std::sync::Arc;
impl codetether_swarm_local_stage::Providers for Native<'_> {
    type Providers = crate::provider::ProviderRegistry;

    fn get(providers: &Self::Providers, name: &str) -> Option<Arc<dyn crate::provider::Provider>> {
        providers.get(name)
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

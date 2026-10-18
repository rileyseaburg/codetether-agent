//! failover provider registry integration.
use crate::{Application, ProviderRegistry};
impl<H: Application> codetether_model_failover::router::Registry for ProviderRegistry<H> {
    fn available(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}

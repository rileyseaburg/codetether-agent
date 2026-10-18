//! Native provider availability for shared failover selection.
impl codetether_model_failover::router::Registry for crate::provider::ProviderRegistry {
    fn available(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}

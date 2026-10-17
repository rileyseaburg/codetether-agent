//! Empty provider registry fixture for the unchanged eligibility tests.
use crate::ModelRegistry;
use codetether_provider_core::Provider;
use std::sync::Arc;
pub(super) struct EmptyRegistry;
impl EmptyRegistry {
    pub(super) fn new() -> Self {
        Self
    }
}
impl ModelRegistry for EmptyRegistry {
    fn get(&self, _: &str) -> Option<Arc<dyn Provider>> {
        None
    }
}

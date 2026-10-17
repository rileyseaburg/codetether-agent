//! with config policy implementation.
use super::*;

impl DelegationState {
    /// Create a fresh state seeded with the supplied config.
    pub fn with_config(config: DelegationConfig) -> Self {
        Self {
            beliefs: BTreeMap::new(),
            config,
        }
    }
}

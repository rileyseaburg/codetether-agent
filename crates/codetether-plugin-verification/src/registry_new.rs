//! registry new for verified plugin registration.
use super::*;
impl PluginRegistry {
    pub fn new(signing_key: SigningKey) -> Self {
        Self {
            signing_key,
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

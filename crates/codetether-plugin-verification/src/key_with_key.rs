//! key with key for verified plugin registration.
use super::*;
impl SigningKey {
    /// Create with an explicit key (for tests).
    #[cfg(test)]
    pub fn with_key(key: Vec<u8>) -> Self {
        Self { key: Arc::new(key) }
    }
}

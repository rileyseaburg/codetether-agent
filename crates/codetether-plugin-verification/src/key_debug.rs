//! key debug for verified plugin registration.
use super::*;

impl std::fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SigningKey")
            .field("key_len", &self.key.len())
            .finish()
    }
}

//! key shared for verified plugin registration.
use super::*;
impl SigningKey {
    /// Load the process-wide signing key exactly once.
    pub fn shared() -> Self {
        GLOBAL_SIGNING_KEY.get_or_init(Self::from_env).clone()
    }
}

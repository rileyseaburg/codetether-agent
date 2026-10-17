//! key for verified plugin registration.
use super::*;

/// The signing key used to verify plugin manifests.
///
/// # Examples
/// ```rust,no_run
/// let _key = codetether_plugin_verification::SigningKey::shared();
/// ```
#[derive(Clone)]
pub struct SigningKey {
    pub(super) key: Arc<Vec<u8>>,
}

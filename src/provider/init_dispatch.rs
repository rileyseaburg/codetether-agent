//! Bind shared provider-secret dispatch to application constructors.
pub(super) mod host;
use crate::{provider::Provider, secrets::ProviderSecrets};
use std::sync::Arc;
/// Construct the provider selected by a Vault entry.
pub fn dispatch(provider_id: &str, secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    codetether_provider_factory::dispatch::<host::Native>(provider_id, secrets)
}

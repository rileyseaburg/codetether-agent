//! Vault-backed provider credentials and authentication maintenance.
//!
//! [`SecretsManager`] owns client state and a shared key cache. [`VaultConfig`]
//! selects an explicit connection; [`login`] resolves saved login profiles.
//! Register a manager with [`init_from_manager`] to use the global helpers.

mod auth_refresh;
mod config;
mod environment;
mod environment_k8s;
mod global;
pub mod login;
mod manager;
mod provider;
mod renewal;
mod retry;
pub use config::VaultConfig;
pub use global::{
    get_api_key, get_provider_secrets, has_api_key, init_from_manager, init_secrets_manager,
    secrets_manager, set_provider_secrets, verify_reachable,
};
pub(crate) use manager::KubernetesAuthConfig;
pub use manager::SecretsManager;
pub use provider::ProviderSecrets;
#[cfg(test)]
mod contract;

mod tenant_keys;
pub use tenant_keys::TenantProviderKeyPayload;

//! Vault client lifecycle, authenticated operations, and key caching.

mod auth_config;
mod cache;
mod client;
mod connect;
mod default;
mod key;
mod kubernetes;
mod kubernetes_login;
mod list;
mod provider;
mod reachable;
mod state;
mod write;
pub(crate) use auth_config::KubernetesAuthConfig;
pub use state::SecretsManager;

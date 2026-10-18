//! Native provider composition for the CodeTether application.
//!
//! [ProviderRegistry] owns the application factory specialization and its
//! process-wide cache. Provider implementations are re-exported from their
//! dedicated crates; [thinker] binds local inference to this same registry.
//! Construct a registry with [ProviderRegistry::new] or load credentials with
//! [ProviderRegistry::from_vault].
mod provider;
pub use provider::*;
pub mod thinker;

/// Cached provider model discovery used for worker registration.
pub mod worker_models;

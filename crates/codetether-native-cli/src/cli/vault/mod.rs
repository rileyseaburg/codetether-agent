//! Shared Vault CLI with full-application integration tests.
pub(crate) use codetether_models_cli::setup as help;
pub use codetether_vault_cli::{VaultArgs, execute};
#[cfg(test)]
mod tests;
#[cfg(test)]
mod workflow_tests;

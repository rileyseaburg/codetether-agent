//! Shared Vault CLI with full-application integration tests.
pub use codetether_vault_cli::{VaultArgs, execute};
pub(crate) mod help;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod workflow_tests;

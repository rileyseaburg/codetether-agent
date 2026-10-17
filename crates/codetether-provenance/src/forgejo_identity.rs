//! Stable Forgejo principal identity derivation and configuration.
//!
//! [forgejo_agent_identity] normalizes the supplied principal and hashes it.
//! Internal configuration readers supply the process's explicitly configured identity.
mod configuration;
mod derive;
mod normalize;
pub(crate) use configuration::{configured, configured_principal};
pub use derive::forgejo_agent_identity;
#[cfg(test)]
mod tests;
